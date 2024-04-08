<script>
  import "../app.pcss";
  import { AppRail, AppRailTile, AppShell, AppBar } from "@skeletonlabs/skeleton";
  import { Modal } from "@skeletonlabs/skeleton";
  import { initializeStores } from "@skeletonlabs/skeleton";
  import { Toast } from "@skeletonlabs/skeleton";
  import Login from "$lib/components/Login.svelte";
  import { goto } from '$app/navigation';
  import { writable } from 'svelte/store';

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
    <AppRail selected={0}>
      <AppRailTile label="Deck" on:click={() => {selected.set(0); goto('/');}} class={$selected === 0 ? 'bg-primary-500 text-white' : ''}>
        <svelte:fragment slot="lead">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
        </svelte:fragment>
      </AppRailTile>

      <AppRailTile label="Wallet" on:click={() => {selected.set(1); goto('/wallet');}} class={$selected === 1 ? 'bg-primary-500 text-white' : ''}>
        <svelte:fragment slot="lead">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
          </svg>
        </svelte:fragment>
      </AppRailTile>

      <AppRailTile label="Marketplace" on:click={() => {selected.set(2); goto('/marketplace');}} class={$selected === 2 ? 'bg-primary-500 text-white' : ''}>
        <svelte:fragment slot="lead">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
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