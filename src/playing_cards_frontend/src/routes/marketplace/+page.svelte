<script>
  import "../../index.scss";
  import { cardBackend, ledgerBackend } from "$lib/canisters/canisters";
  import { onMount } from "svelte";
  import NftCard from "$lib/components/NftCard.svelte";

  let nfts = [];
  onMount(fetchNFTs);

  async function fetchNFTs() {
    nfts = await cardBackend.list_sale_listings();
  }

  async function listNFTForSale() {
    const response = await cardBackend.list_nft_for_sale(
      sellTokenId,
      sellPrice
    );
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
    <div class="grid">
      {#each nfts as nft}
        <div class="nft-card">
          <div class="card" style="width: 18rem;">
            <section class="p-4 flex justify-center">
              <div
                style="width: 250px; height: 250px; background-color: #f0f0f0;"
              ></div>
            </section>
            <header class="card-header">{nft.name}</header>
            <footer class="card-footer">{nft.description}</footer>
            <div class="card-body">
              <button class="btn btn-primary">List for Sale</button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </main>
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
  }
</style>
