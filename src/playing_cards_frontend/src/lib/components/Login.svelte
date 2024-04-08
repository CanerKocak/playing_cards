<script>
  import { AuthClient } from "@dfinity/auth-client";
  import { onMount } from "svelte";
  import { updateBackend } from "$lib/canisters/canisters";
  import { principal, loggedIn } from "$lib/stores/auth";

  let authClient;

  onMount(async () => {
    authClient = await AuthClient.create();
    const isLoggedIn = await authClient.isAuthenticated();
    if (isLoggedIn) {
      principal.set(authClient.getIdentity().getPrincipal().toText());
      loggedIn.set(true);
      updateBackend(authClient.getIdentity());
    }
  });

  async function handleLogin() {
    const identityProvider = process.env.DFX_NETWORK === "ic"
      ? "https://identity.ic0.app/#authorize"
      : `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;

    await authClient.login({
      identityProvider,
      onSuccess: async () => {
        const identity = await authClient.getIdentity();
        principal.set(identity.getPrincipal().toText());
        updateBackend(authClient.getIdentity());
        loggedIn.set(true);
      },
      onError: (error) => {
        console.error("Login error:", error);
      },
    });
  }

  async function handleLogout() {
    await authClient.logout();
    updateBackend();
    principal.set("");
    loggedIn.set(false);
  }
</script>

{#if $loggedIn}
  <button class="btn variant-filled-primary" on:click={handleLogout}>
    Logout
  </button>
{:else}
  <button class="btn variant-filled-primary" on:click={handleLogin}>
    Login with Internet Identity
  </button>
{/if}