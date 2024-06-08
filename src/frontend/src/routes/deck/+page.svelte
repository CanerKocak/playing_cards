<script>
  export const prerender = true;
  import "../../index.scss";
  import { cardBackend } from "$lib/canisters/canisters";
  import { onMount } from "svelte";
  import NftCard from "$lib/components/NftCard.svelte";
  import { getModalStore } from "@skeletonlabs/skeleton";
  const modalStore = getModalStore();

  import { getToastStore } from "@skeletonlabs/skeleton";
  const toastStore = getToastStore();

  let nfts = [];

  onMount(fetchNFTs);

  async function fetchNFTs() {
    nfts = await cardBackend.fetch_nfts();
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
    console.log(event.detail);
  }

  // pop card from nfts array locally
  async function handleRemoveCard(event) {
    const nft = event.detail.nft;
    nfts = nfts.filter((n) => n.id !== nft.id);
  }

  // handle open modal
  async function handleOpenCard(event) {
    const nft = event.detail.nft;
    const modal = {
      type: "modal",
      title: nft.name,
      body: NftCard,
      props: {
        nft,
      },
    };
    modalStore.trigger(modal);
  }
</script>

<main class="container p-4">
  <div class="grid">
    {#each nfts as nft}
      <NftCard
        nft={nft}
        on:remove={handleRemoveCard}
        on:sell={handleNftSell}
        on:burn={handleBurn}
        on:open={handleOpenCard}
      />
    {/each}
  </div>
</main>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
  }
</style>
