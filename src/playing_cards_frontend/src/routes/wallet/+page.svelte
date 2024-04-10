
<script>
  import "../../index.scss";
  import { cardBackend, ledgerBackend } from "$lib/canisters/canisters";
  import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
  import { Principal } from "@dfinity/principal";
  import { principal, loggedIn } from "$lib/stores/auth";
  import { onMount } from "svelte";
  import { clipboard } from "@skeletonlabs/skeleton";

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  let balance = null;
  let formattedBalance = null;
  let recipient = "";
  let amount = 0;
  let isValidPrincipal = false;
  let principleValue = "";

  $: {
    principleValue = $principal;
  }

  $: {
    formattedBalance = formatBalance(balance);
    isValidPrincipal = checkPrincipalValidity(recipient);
  }

  onMount(async () => {
    setTimeout(() => {
      fetchUserBalance();
    }, 100);
  });

  async function fetchUserBalance() {
    if (principleValue === "") {
      return;
    }

    try {
      const principal = Principal.fromText(principleValue);
      if (!principal) {
        console.error("Invalid principal:", principleValue);
        showErrorToast("Invalid user. Please check your input.");
        return;
      }
      balance = await ledgerBackend.icrc1_balance_of({
        owner: Principal.fromText(principleValue),
        subaccount: [],
      });
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
        from_subaccount: [],
        created_at_time: [],
      });

      if ("Ok" in response) {
        console.log("Tokens sent successfully:", response);
        console.log("Tokens sent successfully:", response.Ok);
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
    if (principleValue === "") {
      showErrorToast("Error: User Principal is empty");
      return;
    }

    const userPrincipalShortened =
      principleValue.length > 10
        ? `${principleValue.slice(0, 10)}...`
        : principleValue;
    showSuccessToast(
      `Principal copied to clipboard: ${userPrincipalShortened}`
    );
  }
</script>

<div class="container p-4">
  <header class="mb-8">
    <h1 class="text-4xl font-bold">Wallet Dashboard</h1>
  </header>

  <div class="card p-4 mb-8">
    <h2 class="text-2xl font-semibold mb-4">Account Balance</h2>
    {#if $loggedIn}
      <p class="text-4xl">{formattedBalance || "Loading..."} EXE</p>
      <div class="mt-4 flex items-center">
        <p class="text-lg">Your Principal: {$principal}</p>
      </div>
      <button
        class="btn variant-filled-primary mt-4 mr-2"
        on:click={fetchUserBalance}
      >
        Refresh Balance
      </button>
      <button
        class="btn variant-filled-primary mt-4"
        on:click={copyToClipboard}
        use:clipboard={{ principleValue }}
      >
        Copy Principal
      </button>
    {:else}
      <p class="text-lg font-semibold">Please login to view your balance.</p>
    {/if}
  </div>

  <div class="card p-4">
    <h2 class="text-2xl font-semibold mb-4">Send EXE</h2>
    <form on:submit|preventDefault={sendTokens}>
      <!-- Recipient Address -->
      <div
        class="input-group input-group-divider grid-cols-[1fr_auto] mb-4 relative"
      >
        <input
          type="text"
          id="recipient"
          class="w-full"
          bind:value={recipient}
          placeholder="Enter Principle..."
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
        <div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
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

      <button type="submit" class="btn variant-filled" disabled={!isValidPrincipal}>
        Send EXE
    </form>
  </div>
</div>
