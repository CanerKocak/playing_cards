<script>
  import { onMount } from "svelte";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { Principal } from "@dfinity/principal";
  import { loggedIn } from "$lib/stores/auth";
  import { cardBackend, ledgerBackend } from "$lib/canisters/canisters";
  import { canisterId as cardCanisterId } from "declarations/playing_cards_backend";
  import { formatBigDecimalToString2Digits } from "$lib/utils";
  import Login from "$lib/components/Login.svelte";
  import NftCard from "$lib/components/NftCard.svelte";
  import NftDetails from "$lib/components/NftDetails.svelte";

  let whoami = null;
  let whoamisub = null;
  let walletBalance = 0n;
  let dappBalance = 0n;
  let recipient = "";
  let amount = 0;
  let isValidPrincipal = false;
  let fetchingBalances = false;
  let icrc1_fee = 1_000_000n;
  let nfts = [];
  let selectedNft = null;

  const toastStore = getToastStore();

  $: formattedWalletBalance = formatBalance(walletBalance);
  $: formattedDappBalance = formatBalance(dappBalance);
  $: isValidPrincipal = checkPrincipalValidity(recipient);

  onMount(async () => {
    await initializeUser();
  });

  $: if ($loggedIn) {
    initializeUser();
    fetchBalances();
    fetchNfts();
  }

  async function initializeUser() {
    try {
      whoami = await cardBackend.whoami();
      whoamisub = await cardBackend.whoamisub();
      await fetchBalances();
      await fetchNfts();
    } catch (error) {
      showErrorToast("Failed to initialize user.");
      console.error("Failed to initialize user:", error);
    }
  }

  async function fetchBalances() {
    if (!whoami) return;
    fetchingBalances = true;
    try {
      walletBalance = await getBalance(whoami);
      dappBalance = await getSubaccountBalance(whoamisub);
    } catch (error) {
      showErrorToast("Failed to fetch balances.");
      console.error("Failed to fetch balances:", error);
    } finally {
      fetchingBalances = false;
    }
  }

  async function fetchNfts() {
    try {
      const userNfts = await cardBackend.getMetadataForUserDip721(whoami);
      nfts = userNfts.map((nft) => ({
        ...nft,
        owner: whoami,
      }));
    } catch (error) {
      showErrorToast("Failed to fetch NFTs.");
      console.error("Failed to fetch NFTs:", error);
    }
  }

  async function getBalance(principal) {
    return await ledgerBackend.icrc1_balance_of({
      owner: Principal.fromText(principal.toText()),
      subaccount: [],
    });
  }

  async function getSubaccountBalance(principal) {
    try {
      const whoamisub = await cardBackend.whoamisub();
      return await ledgerBackend.icrc1_balance_of({
        owner: Principal.fromText(cardCanisterId),
        subaccount: [whoamisub],
      });
    } catch (error) {
      showErrorToast("Failed to fetch subaccount balance.");
      console.error("Failed to fetch subaccount balance:", error);
      return 0n;
    }
  }

  async function sendTokens() {
    showSuccessToast("Sending tokens...");
    try {
      const transferAmount = BigInt(Math.round(amount * 1e8)) - icrc1_fee;
      const response = await ledgerBackend.icrc1_transfer({
        to: { owner: Principal.fromText(recipient.trim()), subaccount: [] },
        amount: transferAmount,
        memo: [],
        fee: [],
        from_subaccount: [],
        created_at_time: [],
      });

      if ("Ok" in response) {
        showSuccessToast("Tokens sent successfully!");
        fetchBalances();
        resetForm();
      } else {
        showErrorToast(`Failed to send tokens: ${response.Err}`);
      }
    } catch (error) {
      showErrorToast("Failed to send tokens. Please try again.");
      console.error("Failed to send tokens:", error);
    }
  }

  async function sendToDapp() {
    showSuccessToast("Sending tokens to dApp...");
    try {
      let whoamisub = await cardBackend.whoamisub();
      console.log("whoamisub", whoamisub);

      let account = {
        owner: Principal.fromText(cardCanisterId),
        subaccount: [whoamisub],
      };

      const amountToSend = BigInt(walletBalance) - icrc1_fee;

      const response = await ledgerBackend.icrc1_transfer({
        to: account,
        amount: amountToSend,
        fee: [],
        memo: [],
        from_subaccount: [],
        created_at_time: [],
      });

      console.log(amountToSend);
      console.log(response);
      if ("Ok" in response) {
        showSuccessToast("Tokens sent to dApp successfully!");
        fetchBalances();
      } else {
        showErrorToast(`Failed to send tokens to dApp: ${response.Err}`);
      }
    } catch (error) {
      showErrorToast("Failed to send tokens to dApp. Please try again.");
      console.error("Failed to send tokens to dApp:", error);
    }
  }

  async function sendFromDappToWallet() {
    showSuccessToast("Sending tokens from dApp to wallet...");
    try {
      const response = await cardBackend.send_exe_to_wallet();
      console.log(response);
      if ("Ok" in response) {
        showSuccessToast("Tokens sent from dApp to wallet successfully!");
        fetchBalances();
      } else {
        showErrorToast(
          `Failed to send tokens from dApp to wallet: ${response.Err}`
        );
      }
    } catch (error) {
      showErrorToast(
        "Failed to send tokens from dApp to wallet. Please try again."
      );
      console.error("Failed to send tokens from dApp to wallet:", error);
    }
  }

  function resetForm() {
    recipient = "";
    amount = 0;
  }

  function showSuccessToast(message) {
    toastStore.trigger({
      message,
      background: "variant-filled-primary",
      timeout: 3000,
    });
  }

  function showErrorToast(message) {
    toastStore.trigger({
      message,
      background: "variant-filled-error",
      timeout: 5000,
    });
  }

  function formatBalance(balance) {
    return formatBigDecimalToString2Digits(balance);
  }

  function checkPrincipalValidity(principal) {
    try {
      Principal.fromText(principal);
      return true;
    } catch {
      return false;
    }
  }

  function copyToClipboard(text) {
    navigator.clipboard
      .writeText(text)
      .then(() => {
        showSuccessToast("Copied to clipboard!");
      })
      .catch((error) => {
        showErrorToast("Failed to copy to clipboard.");
        console.error("Failed to copy to clipboard:", error);
      });
  }

  function handleView(event) {
    selectedNft = event.detail.nft;
  }

  function handleUpdate(event) {
    // Handle NFT update logic here
  }
</script>

<div class="container m-5">
  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Wallet & dApp Management</div>
      <Login />
    </div>
    <div class="window-body">
      {#if $loggedIn}
        <div>
          <h2>Wallet Balance (Principal)</h2>
          <p class="field-row">{formattedWalletBalance || "Loading..."} EXE</p>
          <button
            class="button"
            on:click={fetchBalances}
            disabled={fetchingBalances}>Refresh</button
          >
          <button class="button" on:click={sendToDapp}>Send dApp</button>
        </div>
        <div>
          <h2>dApp Balance (Subaccount)</h2>
          <p class="field-row">{formattedDappBalance || "Loading..."} EXE</p>
          <button
            class="button"
            on:click={fetchBalances}
            disabled={fetchingBalances}>Refresh</button
          >
          <button class="button" on:click={sendFromDappToWallet}
            >Send Wallet</button
          >
        </div>
        <div>
          <h2>User Address</h2>
          <div>
            <label>Principal ID:</label>
            <input type="text" value={whoami} readonly />
            <button class="button" on:click={() => copyToClipboard(whoami)}
              >Copy</button
            >
          </div>
          <div>
            <label>Subaccount:</label>
            <input type="text" value={whoamisub} readonly />
            <button class="button" on:click={() => copyToClipboard(whoamisub)}
              >Copy</button
            >
          </div>
        </div>
        <div>
          <h2>Send EXE</h2>
          <form on:submit|preventDefault={sendTokens}>
            <div class="field-row">
              <label for="recipient">Recipient:</label>
              <input
                type="text"
                id="recipient"
                bind:value={recipient}
                placeholder="Enter Principal..."
                required
              />
              <span>{isValidPrincipal ? "✅" : "❌"}</span>
            </div>
            <div class="field-row">
              <label for="amount">Amount:</label>
              <input
                type="number"
                id="amount"
                bind:value={amount}
                min="0.01"
                step="0.00000001"
                required
              />
            </div>
            <button type="submit" class="button" disabled={!isValidPrincipal}
              >Send EXE</button
            >
          </form>
        </div>
        <div>
          <h2>NFTs</h2>
          {#if selectedNft}
            <NftDetails {nft} on:update={handleUpdate} />
          {:else}
            {#each nfts as nft (nft.id)}
              <NftCard {nft} on:view={handleView} />
            {/each}
          {/if}
        </div>
      {:else}
        <p>Please login to view your balance.</p>
      {/if}
    </div>
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(700px, 1fr));
    gap: 1rem;
    margin: 20px;
  }

  .window {
    border: 1px solid #ccc;
    border-radius: 5px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    height: 100%;
  }

  .title-bar {
    background-color: #f0f0f0;
    padding: 10px;
    border-bottom: 1px solid #ccc;
  }

  .title-bar-text {
    font-weight: bold;
    font-size: 16px;
  }

  .window-body {
    padding: 15px;
    font-size: 14px;
    line-height: 1.6;
  }

  img {
    max-width: 100%;
    height: auto;
    margin-top: 10px;
  }
</style>
