<script>
  export const prerender = true;
  import "../app.pcss";
  import { AppRail, AppRailTile, AppShell, AppBar } from "@skeletonlabs/skeleton";
  import { Modal } from "@skeletonlabs/skeleton";
  import { initializeStores } from "@skeletonlabs/skeleton";
  import { Toast } from "@skeletonlabs/skeleton";
  import Login from "$lib/components/Login.svelte";
  import { goto } from '$app/navigation';
  import { writable } from 'svelte/store';

  // Import wallet svg
  import wallet from "../../../../wallet.svg";
  import deck from "../../../../deck.svg";
  import marketplace from "../../../../marketplace.svg";

  // windoge98 logo
  import logo from "../../../../windoge98_logo.png";

  initializeStores();

  let currentTile = writable(0);
</script>

<Toast />
<Modal />

<AppShell>
  <svelte:fragment slot="header">
        <div class="window">
          <div class="title-bar">
            <div class="title-bar-text">Windoge98</div>
            <div class="title-bar-controls">
              <button aria-label="Close"></button>
            </div>
          </div>
          <div class="flex justify-between items-center m-1">
            <div class="flex items-center">
              <img src={logo} alt="Windoge98 logo" class="w-20 h-auto mr-4 ml -3" />
              <h1 class="text-3xl font-bold">Playing Cards Collection</h1>
            </div>
            <Login />
          </div>
        </div>
  </svelte:fragment>

  <svelte:fragment slot="sidebarLeft">
    <AppRail>
      <AppRailTile bind:group={$currentTile} name="deck" value={0} title="Deck" on:click={() => goto('/')}>
        <svelte:fragment slot="lead">
          <img src={deck} alt="Deck" class="icon" />
        </svelte:fragment>
        <span>Deck</span>
      </AppRailTile>
      <AppRailTile bind:group={$currentTile} name="wallet" value={1} title="Wallet" on:click={() => goto('/wallet')}>
        <svelte:fragment slot="lead">
          <img src={wallet} alt="Wallet" class="icon" />
        </svelte:fragment>
        <span>Wallet</span>
      </AppRailTile>
      <AppRailTile bind:group={$currentTile} name="marketplace" value={2} title="Marketplace" on:click={() => goto('/marketplace')}>
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
  .icon {
    width: 32px;
    height: 32px;
    margin-left: 24px;
  }
</style>