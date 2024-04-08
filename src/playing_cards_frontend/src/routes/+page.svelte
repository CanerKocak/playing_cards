<script>
  import "../index.scss";
  import { backend } from "$lib/canisters/canisters";
  import { onMount } from "svelte";
  import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
  import { Principal } from "@dfinity/principal";

  const modalStore = getModalStore();
  const toastStore = getToastStore();

  let balance = null;
  let formattedBalance = null;
  let recipient = "";
  let amount = 0;
  let isValidPrincipal = false;
  let userPrincipal = "";

  onMount(() => {
    setTimeout(fetchUserBalance, 100); // allow internet identity to load
  });

  $: {
    formattedBalance = formatBalance(balance);
    isValidPrincipal = checkPrincipalValidity(recipient);
  }

  async function fetchUserBalance() {
    showSuccessToast("Fetching user balance...");
    try {
      const result = await backend.user_balance();
      if ("Ok" in result) {
        balance = result.Ok;
        console.log("User balance:11", balance);
        fetchUserPrincipal();
        showSuccessToast("User balance fetched successfully!");
        
      } else {
        console.error("Error fetching user balance:", result.Err);
        showErrorToast("Failed to fetch user balance. Please try again.");
      }
    } catch (error) {
      console.error("Error fetching user balance:", error);
      showErrorToast("Failed to fetch user balance. Please try again.");
    }
  }

  async function fetchUserPrincipal() {
    let Principal = await backend.whoami_string();
    userPrincipal = Principal;
  }


  async function sendTokens() {
    showSuccessToast("Sending tokens...");
    try {
      const response = await backend.transfer_from_caller({
        to_account: {
          owner: Principal.fromText(recipient),
          subaccount: [],
        },
        amount: BigInt(amount * 100000000),
      });

      if ("Ok" in response) {
        console.log("Tokens sent successfully:", response);
        console.log("Tokens sent successfully:", response.Ok);
        showSuccessToast("Tokens sent successfully!");
        fetchUserBalance();
        resetForm();
      } else {
        console.error("Error sending tokens:", response.Err);
        showErrorToast(`Failed to send tokens: ${response.Err}`);
      }
    } catch (error) {
      console.error("Error sending tokens:", error);
      if (error.message.includes("ledger transfer error")) {
        const errorMessage = error.message.split("ledger transfer error")[1].trim();
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

    // Pad the string with leading zeros to ensure it has at least 9 characters
    // This accounts for 8 decimal places plus at least one digit before the decimal point
    balanceStr = balanceStr.padStart(9, "0");

    // Insert the decimal point eight places from the end
    let formattedBalance = balanceStr.slice(0, -8) + "." + balanceStr.slice(-8);

    // Remove trailing zeros after the decimal point to clean up the display
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
    if (userPrincipal === "") {
      showErrorToast("Error: User Principal is empty");
      return;
    }

    await navigator.clipboard.writeText(userPrincipal);

    const userPrincipalShortened =
      userPrincipal.length > 10
        ? `${userPrincipal.slice(0, 10)}...`
        : userPrincipal;
      
    showSuccessToast(`Principal copied to clipboard: ${userPrincipalShortened}`);
  }
</script>

<div class="container p-4">
  <header class="mb-8">
    <h1 class="text-4xl font-bold">Wallet Dashboard</h1>
  </header>

  <div class="card p-4 mb-8">
    <h2 class="text-2xl font-semibold mb-4">Account Balance</h2>
    <p class="text-4xl">{formattedBalance || "Loading..."} EXE</p>
    <div class="mt-4 flex items-center">
      <p class="text-lg">Your Principal: {userPrincipal}</p>
    </div>
    <button class="btn variant-filled-primary mt-4 mr-2" on:click={fetchUserBalance}>
      Refresh Balance
    </button>

    <button class="btn variant-filled-primary mt-4" on:click={copyToClipboard}>
      Copy Principal
    </button>
  </div>

  <div class="card p-4">
    <h2 class="text-2xl font-semibold mb-4">Send EXE</h2>
    <form on:submit|preventDefault={sendTokens}>
      <!-- Recipient Address -->
      <div class="input-group input-group-divider grid-cols-[1fr_auto] mb-4 relative">
        <input
          type="text"
          id="recipient"
          class="w-full"
          bind:value={recipient}
          placeholder="Enter Principle..."
          required
        />
        {#if isValidPrincipal}
          <span title="Principal is valid" class="absolute right-4 top-1/2 transform -translate-y-1/2">✅</span>
        {:else}
          <span title="Principal is invalid" class="absolute right-4 top-1/2 transform -translate-y-1/2">❌</span>
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
            min="0"
            step="0.00000001"
            required
          />
        </div>
      </div>

      <button type="submit" class="btn variant-filled">Send</button>
    </form>
  </div>
</div>
