<script>
  import "../index.scss";
  import { backend } from "$lib/canisters/canisters";
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
    nfts = await backend.listAllNftsFull();
  }

  async function mintNFT() {
    const response = await backend.mintDip721(
      mintPrincipal,
      mintMetadata,
      mintContent
    );
    if (response.Ok) {
      console.log("NFT minted successfully:", response.Ok);
      await fetchNFTs();
    } else {
      console.error("Error minting NFT:", response.Err);
    }
  }

  function handleFileUpload(event) {
    const file = event.target.files[0];
    const reader = new FileReader();
    reader.onload = () => {
      mintContent = new Uint8Array(reader.result);
    };
    reader.readAsArrayBuffer(file);
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

  async function getCallerInfo() {
    const response = await backend.get_caller();
    if (response.Ok) {
      console.log("Caller info:", response.Ok);
    } else {
      console.error("Error getting caller info:", response.Err);
    }
  }
</script>

<div class="container p-4">
  <main>
    <button class="btn variant-filled" on:click={getCallerInfo}>
      Get Caller Info
    </button>
    <div class="grid grid-cols-4 gap-4">
      {#each nfts as nft}
        <div class="nft-card">
          <NftCard {nft} on:sell={handleNftSell} />
        </div>
      {/each}
    </div>

    <div class="mt-8">
      <h2 class="text-2xl font-semibold mb-4">Mint a New NFT</h2>
      <form on:submit|preventDefault={mintNFT}>
        <div class="mb-4">
          <label for="mintPrincipal" class="block mb-2">Principal:</label>
          <input
            type="text"
            id="mintPrincipal"
            bind:value={mintPrincipal}
            class="input"
          />
        </div>
        <div class="mb-4">
          <label for="mintMetadata" class="block mb-2">Metadata:</label>
          <textarea id="mintMetadata" bind:value={mintMetadata} class="textarea"
          ></textarea>
        </div>
        <div class="mb-4">
          <label for="mintContent" class="block mb-2">Content:</label>
          <input
            type="file"
            id="mintContent"
            on:change={handleFileUpload}
            class="file-input"
          />
        </div>
        <button type="submit" class="btn variant-filled">Mint NFT</button>
      </form>
    </div>
  </main>
</div>
