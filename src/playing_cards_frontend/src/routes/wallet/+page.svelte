<script>
  import { onMount } from "svelte";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { Principal } from "@dfinity/principal";
  import { loggedIn } from "$lib/stores/auth";
  import { cardBackend, ledgerBackend } from "$lib/canisters/canisters";
  import { canisterId as cardCanisterId } from "declarations/playing_cards_backend";
  import { formatBigDecimalToString2Digits, shortenCaller } from "$lib/utils";

  let whoami = null;
  let whoamisub = null;
  let walletBalance = 0n;
  let dappBalance = 0n;
  let recipient = "";
  let amount = 0;
  let isValidPrincipal = false;
  let fetchingBalances = false;
  let icrc1_fee = 1_000_000n;

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
  }

  async function initializeUser() {
    try {
      whoami = await cardBackend.whoami();
      whoamisub = await cardBackend.whoamisub();
      await fetchBalances();
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
</script>

<div class="container">
  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Wallet Balance (Principal)</div>
    </div>
    <div class="window-body">
      {#if $loggedIn}
        <p class="balance">{formattedWalletBalance || "Loading..."} EXE</p>
        <button
          on:click={fetchBalances}
          class="refresh-btn"
          disabled={fetchingBalances}>Refresh</button
        >
      {:else}
        <p class="info-text">Please login to view your balance.</p>
      {/if}
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">dApp Balance (Subaccount)</div>
    </div>
    <div class="window-body">
      {#if $loggedIn}
        <p class="balance">{formattedDappBalance || "Loading..."} EXE</p>
        <button
          on:click={fetchBalances}
          class="refresh-btn"
          disabled={fetchingBalances}>Refresh</button
        >
      {:else}
        <p class="info-text">Please login to view your balance.</p>
      {/if}
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">User Address</div>
    </div>
    <div class="window-body">
      {#if $loggedIn}
        <div class="address-info">
          <p class="label">Principal ID:</p>
          <div class="input-group">
            <input type="text" value={whoami} readonly class="input" />
            <button on:click={() => copyToClipboard(whoami)} class="copy-btn"
              >Copy</button
            >
          </div>
          <p class="label">Subaccount:</p>
          <div class="input-group">
            <input type="text" value={whoamisub} readonly class="input" />
            <button on:click={() => copyToClipboard(whoamisub)} class="copy-btn"
              >Copy</button
            >
          </div>
        </div>
      {:else}
        <p class="info-text">Please login to view your address.</p>
      {/if}
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Send EXE</div>
    </div>
    <div class="window-body">
      <form on:submit|preventDefault={sendTokens} class="send-form">
        <div class="input-group">
          <input
            type="text"
            id="recipient"
            class="input"
            bind:value={recipient}
            placeholder="Enter Principal..."
            required
          />
          <span
            class="validity-indicator"
            title={isValidPrincipal
              ? "Principal is valid"
              : "Principal is invalid"}
          >
            {isValidPrincipal ? "✅" : "❌"}
          </span>
        </div>
        <div class="input-group">
          <label class="input-label" for="amount">EXE</label>
          <input
            type="number"
            id="amount"
            bind:value={amount}
            min="0.01"
            step="0.00000001"
            required
            class="input"
          />
        </div>
        <button type="submit" class="send-btn" disabled={!isValidPrincipal}
          >Send EXE</button
        >
        <button type="button" class="send-btn" on:click={sendToDapp}
          >Send to dApp</button
        >
        <button type="button" class="send-btn" on:click={sendFromDappToWallet}
          >Send from dApp to Wallet</button
        >
      </form>
    </div>
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
  }
</style>
