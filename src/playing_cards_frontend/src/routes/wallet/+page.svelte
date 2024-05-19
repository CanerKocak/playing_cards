<script>
  import "../../index.scss";
  import { cardBackend, ledgerBackend } from "$lib/canisters/canisters";
  import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
  import { Principal } from "@dfinity/principal";
  import { loggedIn, principal } from "$lib/stores/auth";
  import { onMount } from "svelte";
  import * as crypto from "crypto-js"; // Importing from crypto-js

  let nfts = [];
  let whoami = null;
  let whoamisub = null;
  let balance = null;
  let formattedBalance = null;
  let recipient = "";
  let amount = 0;
  let isValidPrincipal = false;

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  $: {
    formattedBalance = formatBalance(balance);
    isValidPrincipal = checkPrincipalValidity(recipient);
  }

  onMount(async () => {
    await getWhoAmI();
    fetchNFTs();
    setTimeout(() => {
      fetchUserBalance();
    }, 100);
  });

  async function fetchNFTs() {
    nfts = await cardBackend.list_sale_listings();
  }

  async function getWhoAmI() {
    whoami = await cardBackend.whoami();
    whoamisub = crypto.enc.Hex.stringify(crypto.lib.WordArray.create(whoami));
  }

  async function fetchUserBalance() {
    if (!whoamisub) {
      await getWhoAmI();
    }

    try {
      const user = await cardBackend.get_user($principal.toText());
      if (user) {
        whoamisub = crypto.enc.Hex.stringify(
          crypto.lib.WordArray.create(user.subaccount)
        );
        balance = user.balance;
      } else {
        await cardBackend.register_user($principal.toText());
        await getWhoAmI();
        balance = 0;
      }
    } catch (error) {
      showErrorToast("Failed to fetch user balance. Please try again.");
    }
  }

  async function sendTokens() {
    showSuccessToast("Sending tokens...");
    try {
      const response = await ledgerBackend.icrc1_transfer({
        to: { owner: Principal.fromText(recipient.trim()), subaccount: [] },
        amount: BigInt(Math.round(amount * 1e8)),
        memo: [],
        fee: [],
        from_subaccount: [Uint8Array.from(Buffer.from(whoamisub, "hex"))], // Convert hex string to Uint8Array
        created_at_time: [],
      });

      if ("Ok" in response) {
        showSuccessToast("Tokens sent successfully!");
        fetchUserBalance();
        resetForm();
      } else {
        showErrorToast(`Failed to send tokens: ${response.Err}`);
      }
    } catch (error) {
      if (error.message.includes("ledger transfer error")) {
        const errorMessage = error.message
          .split("ledger transfer error")[1]
          .trim();
        showErrorToast(`Failed to send tokens: ${errorMessage}`);
      } else {
        showErrorToast("Failed to send tokens. Please try again.");
      }
    }
  }

  function resetForm() {
    recipient = "";
    amount = 0;
  }

  function showSuccessToast(message) {
    const successToast = {
      message,
      background: "variant-filled-primary",
      timeout: 3000,
    };
    toastStore.trigger(successToast);
  }

  function showErrorToast(message) {
    const errorToast = {
      message,
      background: "variant-filled-error",
      timeout: 5000,
    };
    toastStore.trigger(errorToast);
  }

  function formatBalance(balance) {
    if (balance === null) return "Loading...";
    let balanceStr = balance.toString();
    balanceStr = balanceStr.padStart(9, "0");
    let formattedBalance = balanceStr.slice(0, -8) + "." + balanceStr.slice(-8);
    formattedBalance = parseFloat(formattedBalance).toString();
    return formattedBalance;
  }

  function checkPrincipalValidity(principal) {
    try {
      Principal.fromText(principal);
      return true;
    } catch (error) {
      return false;
    }
  }

  async function copyToClipboard() {
    if (!whoamisub) {
      showErrorToast("Error: User Subaccount is empty");
      return;
    }

    const userSubaccountShortened =
      whoamisub.length > 10 ? `${whoamisub.slice(0, 10)}...` : whoamisub;
    showSuccessToast(
      `Subaccount copied to clipboard: ${userSubaccountShortened}`
    );
    await navigator.clipboard.writeText(whoamisub); // Copy to clipboard
  }
</script>

<div class="container p-4">
  <div class="window mb-4">
    <div class="title-bar">
      <div class="title-bar-text">Account Balance</div>
      <div class="title-bar-controls">
        <button aria-label="Close"></button>
      </div>
    </div>
    {#if $loggedIn}
      <p class="text-4xl p-3">{formattedBalance || "Loading..."} EXE</p>
      <div class="mt-4 flex items-center">
        <p class="text-lg p-3">Your Subaccount: {whoamisub}</p>
      </div>
      <button
        class="btn variant-filled-primary mt-4 mr-2"
        on:click={fetchUserBalance}
      >
        Refresh
      </button>
      <button
        class="btn variant-filled-primary mt-4"
        on:click={copyToClipboard}
      >
        Copy
      </button>
    {:else}
      <p class="text-lg font-semibold p-3">
        Please login to view your balance.
      </p>
    {/if}
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Send EXE</div>
      <div class="title-bar-controls">
        <button aria-label="Close"></button>
      </div>
    </div>
    <form on:submit|preventDefault={sendTokens} class="p-4">
      <!-- Recipient Address -->
      <div
        class="input-group input-group-divider grid-cols-[1fr_auto] mb-4 mt-4 p-2 relative"
      >
        <input
          type="text"
          id="recipient"
          class="w-full"
          bind:value={recipient}
          placeholder="Enter Principal..."
          required
        />
        {#if isValidPrincipal}
          <span
            title="Principal is valid"
            class="absolute right-4 top-1/2 transform -translate-y-1/2">✅</span
          >
        {:else}
          <span
            title="Principal is invalid"
            class="absolute right-4 top-1/2 transform -translate-y-1/2">❌</span
          >
        {/if}
      </div>

      <!-- Amount of EXE -->
      <div class="mb-4">
        <div
          class="input-group input-group-divider grid-cols-[auto_1fr_auto] p-1"
        >
          <div class="input-group-shim">EXE</div>
          <input
            type="number"
            id="amount"
            bind:value={amount}
            min="0.01"
            step="0.00000001"
            required
          />
        </div>
      </div>

      <button
        type="submit"
        class="btn variant-filled"
        disabled={!isValidPrincipal}
      >
        Send EXE
      </button>
    </form>
  </div>

  <main>
    {#if nfts.length > 0}
      <div class="grid">
        {#each nfts as nft}
          <div class="nft-card">
            <div class="card" style="width: 18rem;">
              <section class="p-4 flex justify-center">
                <div
                  style="width: 250px; height: 250px; background-color: #f0f0f0;"
                ></div>
              </section>
              <div class="card-body">
                <div class="flex justify-between">
                  <button class="btn variant-filled-primary">Send</button>
                  <button class="btn variant-filled-primary">Customize</button>
                  <button class="btn variant-filled-primary">View</button>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <p class="text-lg font-semibold p-3">
        You don't have any cards. Try to see if you can buy one.
      </p>
    {/if}
  </main>
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
  }
</style>
