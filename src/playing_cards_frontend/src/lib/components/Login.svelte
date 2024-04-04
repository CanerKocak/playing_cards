<script>
  import { AuthClient } from "@dfinity/auth-client";
  import { onMount } from "svelte";

  let authClient;
  let isLoggedIn = false;
  let principal = "";

  onMount(async () => {
    authClient = await AuthClient.create();
    isLoggedIn = await authClient.isAuthenticated();

    if (isLoggedIn) {
      principal = (await authClient.getIdentity()).getPrincipal().toText();
    }
  });

  async function handleLogin() {
    const identityProvider =
      process.env.DFX_NETWORK === "ic"
        ? "https://identity.ic0.app/#authorize"
        : `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;

    await authClient.login({
      identityProvider,
      onSuccess: async () => {
        isLoggedIn = true;
        principal = (await authClient.getIdentity()).getPrincipal().toText();
      },
      onError: (error) => {
        console.error("Login error:", error);
      },
    });
  }

  async function handleLogout() {
    await authClient.logout();
    isLoggedIn = false;
    principal = "";
  }
</script>

<div class="login-container">
  {#if isLoggedIn}
    <p>Logged in as: {principal}</p>
    <button class="variant-fill" on:click={handleLogout}> Logout </button>
  {:else}
    <button class="variant-fill" on:click={handleLogin}>
      Login with Internet Identity
    </button>
  {/if}

    <button class="variant-fill" on:click={() => console.log(authClient)}>Auth Info</button>

</div>
