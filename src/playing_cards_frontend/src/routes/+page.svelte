<script>
  export const prerender = true;
  import "../index.scss";
  import { cardBackend } from "$lib/canisters/canisters";
  import { onMount } from "svelte";
  import NftCard from "$lib/components/NftCard.svelte";
  import { getModalStore } from "@skeletonlabs/skeleton";
  const modalStore = getModalStore();
  import card from "../../../../hearts 4.png";

  import { getToastStore } from "@skeletonlabs/skeleton";
  const toastStore = getToastStore();

  let nfts = [];

  onMount(fetchNFTs);

  async function fetchNFTs() {
    nfts = await cardBackend.list_all_nfts_full();
  }

  async function handleNftSell(event) {
    const nft = event.detail.nft;
    nft.name = "test";

    const priceModal = {
      type: "prompt",
      title: "Enter Listing Price (EXE)",
      body: "Please enter the price at which you want to list the NFT:",
      value: "",
      valueAttr: {
        type: "number",
        min: 0,
        required: true,
      },
      response: (price) => {
        if (price === false || price === undefined) {
          // Show cancel toast
          const cancelToast = {
            message: "Listing canceled!",
            background: "variant-filled-warning",
            timeout: 3000,
          };
          toastStore.trigger(cancelToast);
          return;
        }
        const confirmationModal = {
          type: "confirm",
          title: "Confirmation",
          body: `Are you sure you want to list the NFT for ${price} EXE??`,
          response: (confirmed) => {
            if (confirmed !== false && confirmed !== undefined) {
              listNftForSale(nft, price);
            } else {
              // Show cancel toast
              const cancelToast = {
                message: "Listing canceled!",
                background: "variant-filled-warning",
                timeout: 3000,
              };
              toastStore.trigger(cancelToast);
            }
          },
        };
        modalStore.trigger(confirmationModal);
      },
    };
    modalStore.trigger(priceModal);
  }

  async function listNftForSale(nft, sellPrice) {
    const sellTokenId = nft.id;
    console.log("Token ID:", sellTokenId);
    const response = await cardBackend.listAllNftsFull(sellTokenId, sellPrice);
    if (response.Ok) {
      console.log("NFT listed for sale successfully:", response.Ok);
      await fetchNFTs();

      // Show success toast
      const successToast = {
        message: "NFT listed for sale successfully!",
        background: "variant-filled-primary",
        timeout: 3000,
      };
      toastStore.trigger(successToast);
    } else {
      console.error("Error listing NFT for sale:", response.Err);

      // Show error toast
      const errorToast = {
        message: "Error listing NFT for sale. Please try again.",
        background: "variant-filled-error",
        timeout: 5000,
      };
      toastStore.trigger(errorToast);
    }
  }

  async function handleBurn(event) {
    const nft = event.detail.nft;
    const response = await cardBackend.burnDip721(0);
    if (response.Ok) {
      console.log("NFT burned successfully:", response.Ok);
      await fetchNFTs();

      // Show success toast
      const successToast = {
        message: "NFT burned successfully!",
        background: "variant-filled-primary",
        timeout: 3000,
      };
      toastStore.trigger(successToast);
    } else {
      console.error("Error burning NFT:", response.Err);

      // Show error toast
      const errorToast = {
        message: "Error burning NFT. Please try again.",
        background: "variant-filled-error",
        timeout: 5000,
      };
      toastStore.trigger(errorToast);
    }
  }
</script>

<main class="container p-4">
  <div class="grid">
    {#each nfts as nft}
      <div class="nft-card">
        <div class="card" style="width: 18rem;">
          <header class="card-header">{nft.name}</header>
          <footer class="card-footer">{nft.description}</footer>
          <img src={card} alt={nft.name} />
          <button class="btn btn-primary" on:click={handleBurn}>List for Sale</button>
          <div class="card-body">
            <button class="btn btn-primary">List for Sale</button>
          </div>
        </div>
      </div>
    {/each}
  </div>
</main>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
  }
</style>
