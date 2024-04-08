<script>
  import { AuthClient } from "@dfinity/auth-client";
  import { onMount } from "svelte";
  import { backend, updateBackend } from "$lib/canisters/canisters";

  let authClient;
  let isLoggedIn = false;
  let principal = "";

  onMount(async () => {
    authClient = await AuthClient.create();
    isLoggedIn = await authClient.isAuthenticated();

    if (isLoggedIn) {
      principal = authClient.getIdentity().getPrincipal().toText();
      updateBackend(authClient.getIdentity());
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
        const identity = await authClient.getIdentity();
        principal = identity.getPrincipal().toText();
        updateBackend(authClient.getIdentity());
      },
      onError: (error) => {
        console.error("Login error:", error);
      },
    });
  }

  async function handleLogout() {
    await authClient.logout();
    updateBackend(); // Reset the backend
    isLoggedIn = false;
    console.log("Logged out");
    principal = "";
  }
</script>

{#if isLoggedIn}
  <button class="btn variant-filled-primary" on:click={handleLogout}>
    Logout
  </button>
{:else}
  <button class="btn variant-filled-primary" on:click={handleLogin}>
    Login with Internet Identity
  </button>
{/if}
