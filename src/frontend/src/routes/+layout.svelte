<script>
  import { writable } from 'svelte/store';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import "../app.pcss";
  import { AppRail, AppRailTile, AppShell, Modal, Toast } from "@skeletonlabs/skeleton";
  import { initializeStores } from "@skeletonlabs/skeleton";
  import wallet from "/wallet.svg";
  import deck from "/deck.svg";
  import marketplace from "/marketplace.svg";
  import logo from "/windoge98_logo.png";

  let currentTile = writable(0);

  initializeStores();

  $: $page.url.pathname, setCurrentTile();

  function setCurrentTile() {
    switch ($page.url.pathname) {
      case '/':
        currentTile.set(0);
        break;
      case '/deck':
        currentTile.set(1);
        break;
      case '/wallet':
        currentTile.set(2);
        break;
      case '/marketplace':
        currentTile.set(3);
        break;
      default:
        currentTile.set(0); // Default or unknown paths
        break;
    }
  }

  // Navigate and update tile
  function navigateAndSetTile(url, tileIndex) {
    goto(url);
    currentTile.set(tileIndex);
  }
</script>

<Toast />
<Modal />

<AppShell>
  <svelte:fragment slot="sidebarLeft">
    <AppRail>
      <AppRailTile bind:group={$currentTile} name="main" value={0} title="Main" on:click={() => navigateAndSetTile('/', 0)}>
        <svelte:fragment slot="lead">
          <img src={logo} alt="Main" class="logo" />
        </svelte:fragment>
      </AppRailTile>
      <AppRailTile bind:group={$currentTile} name="deck" value={1} title="Deck" on:click={() => navigateAndSetTile('/deck', 1)}>
        <svelte:fragment slot="lead">
          <img src={deck} alt="Deck" class="icon" />
        </svelte:fragment>
        <span>Deck</span>
      </AppRailTile>
      <AppRailTile bind:group={$currentTile} name="wallet" value={2} title="Wallet" on:click={() => navigateAndSetTile('/wallet', 2)}>
        <svelte:fragment slot="lead">
          <img src={wallet} alt="Wallet" class="icon" />
        </svelte:fragment>
        <span>Wallet</span>
      </AppRailTile>
      <AppRailTile bind:group={$currentTile} name="marketplace" value={3} title="Marketplace" on:click={() => navigateAndSetTile('/marketplace', 3)}>
        <svelte:fragment slot="lead">
          <img src={marketplace} alt="Marketplace" class="icon" />
        </svelte:fragment>
        <span>Market</span>
      </AppRailTile>
    </AppRail>
  </svelte:fragment>

  <slot />

  <footer class="mt-8">
    <p class="text-center text-gray-500">
      &copy; 2024 - All rights reserved
    </p>
  </footer>
</AppShell>

<style>
  .icon, .logo {
    width: 32px;
    height: 32px;
    margin-left: 24px;
  }

  .logo {
    width: 64px;
    height: auto;
    margin-left: 8px;
    margin-right: 0;
  }
</style>
