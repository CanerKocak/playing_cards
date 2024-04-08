<script>
  import "../index.scss";
  import { cardBackend } from "$lib/canisters/canisters";
  import { onMount } from "svelte";
  import NftCard from "$lib/components/NftCard.svelte";
  import { getModalStore } from "@skeletonlabs/skeleton";
  const modalStore = getModalStore();

  import { getToastStore } from "@skeletonlabs/skeleton";
  const toastStore = getToastStore();

  let nfts = [];
  let mintPrincipal = "";
  let mintMetadata = [];
  let mintContent = new Uint8Array();

  onMount(fetchNFTs);

  async function fetchNFTs() {
    nfts = await cardBackend.listAllNftsFull();
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
    const response = await backend.list_nft_for_sale(sellTokenId, sellPrice);
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
</script>

<div class="container p-4">
  <main>
    <div class="grid grid-cols-4 gap-4">
      {#each nfts as nft}
        <div class="nft-card">
          <NftCard {nft} on:sell={handleNftSell} />
        </div>
      {/each}
    </div>
</div>