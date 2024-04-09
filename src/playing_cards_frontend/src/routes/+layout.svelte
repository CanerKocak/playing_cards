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

  // import wallet svg
  import wallet from "../../../../wallet.svg";
  import deck from "../../../../deck.svg";
  import marketplace from "../../../../marketplace.svg";

  initializeStores();

  let selected = writable(0);

</script>

<Toast />
<Modal />

<AppShell>
  <svelte:fragment slot="header">
    <AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
      <svelte:fragment slot="lead">
        <h1 class="text-2xl font-bold">Playing Cards Collection</h1>
      </svelte:fragment>
      <svelte:fragment slot="trail">
        <Login />
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>

  <svelte:fragment slot="sidebarLeft">
    <AppRail>
      <AppRailTile label="Deck" on:click={() => {selected.set(0); goto('/');}} class="bg-primary-500 text-white">
        <svelte:fragment slot="lead">
          <img src={deck} alt="Deck" />
          <span class="text-sm">Deck</span>
        </svelte:fragment>
      </AppRailTile>

      <AppRailTile label="Wallet" on:click={() => {selected.set(1); goto('/wallet');}} class={$selected === 1 ? 'bg-primary-500 text-white' : 'bg-primary-200 text-black'}>
        <svelte:fragment slot="lead">
          <img src={wallet} alt="Wallet" />
          <span class="text-sm">Wallet</span>
        </svelte:fragment>
      </AppRailTile>

      <AppRailTile label="Marketplace" on:click={() => {selected.set(2); goto('/marketplace');}} class={$selected === 2 ? 'bg-primary-500 text-white' : ''}>
        <svelte:fragment slot="lead">
          <img src={marketplace} alt="Marketplace" />
          <span class="text-sm">Marketplace</span>
        </svelte:fragment>
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