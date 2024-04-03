<script>
  import "../index.scss";
  import { backend } from "$lib/canisters/canisters";
  import { onMount } from "svelte";
  import NftCard from "$lib/components/NftCard.svelte";

  let nfts = [];
  let mintPrincipal = "";
  let mintMetadata = [];
  let mintContent = new Uint8Array();
  let sellTokenId = 0;
  let sellPrice = 0;

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

  async function listNFTForSale() {
    const response = await backend.list_nft_for_sale(sellTokenId, sellPrice);
    if (response.Ok) {
      console.log("NFT listed for sale successfully:", response.Ok);
      await fetchNFTs();
    } else {
      console.error("Error listing NFT for sale:", response.Err);
    }
  }
</script>

<div class="container p-4">
  <main>
    <div class="grid grid-rows-10 gap-1">
      {#each Array(6) as _, rowIndex}
        <div class="flex overflow-x-auto snap-x snap-mandatory scroll-pl-4">
          {#each nfts.slice(rowIndex * 8, (rowIndex + 1) * 8) as nft}
            <div class="nft-card flex-shrink-0 mr-4 mb-4">
              <NftCard {nft} />
            </div>
          {/each}
        </div>
      {/each}
    </div>

    <div class="mt-8">
      <h2 class="text-2xl font-semibold mb-4">Mint a New NFT</h2>
      <form on:submit|preventDefault={mintNFT}>
        <div class="mb-4">
          <label for="mintPrincipal" class="block mb-2">Principal:</label>
          <input type="text" id="mintPrincipal" bind:value={mintPrincipal} class="input" />
        </div>
        <div class="mb-4">
          <label for="mintMetadata" class="block mb-2">Metadata:</label>
          <textarea id="mintMetadata" bind:value={mintMetadata} class="textarea"></textarea>
        </div>
        <div class="mb-4">
          <label for="mintContent" class="block mb-2">Content:</label>
          <input type="file" id="mintContent" on:change={handleFileUpload} class="file-input" />
        </div>
        <button type="submit" class="btn variant-filled">Mint NFT</button>
      </form>
    </div>

    <div class="mt-8">
      <h2 class="text-2xl font-semibold mb-4">Sell an NFT</h2>
      <form on:submit|preventDefault={listNFTForSale}>
        <div class="mb-4">
          <label for="sellTokenId" class="block mb-2">Token ID:</label>
          <input type="number" id="sellTokenId" bind:value={sellTokenId} class="input" />
        </div>
        <div class="mb-4">
          <label for="sellPrice" class="block mb-2">Price:</label>
          <input type="number" id="sellPrice" bind:value={sellPrice} class="input" />
        </div>
        <button type="submit" class="btn variant-filled">Sell NFT</button>
      </form>
    </div>
  </main>

  <footer class="mt-8">
    <p class="text-center text-gray-500">
      &copy; 2024 My NFT Project. All rights reserved.
    </p>
  </footer>
</div>