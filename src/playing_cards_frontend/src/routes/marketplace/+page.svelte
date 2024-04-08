<script>
  import "../../index.scss";
  import { cardBackend, ledgerBackend } from "$lib/canisters/canisters";
  import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";
  import { Principal } from "@dfinity/principal";
  import { principal, loggedIn } from "$lib/stores/auth";
  import { onMount } from "svelte";
	import NftCard from "$lib/components/NftCard.svelte";

	let nfts = [];
	onMount(fetchNFTs);

	async function fetchNFTs() {
		nfts = await cardBackend.listAllNftsFull();
	}

	async function listNFTForSale() {
    const response = await cardBackend.list_nft_for_sale(sellTokenId, sellPrice);
    if (response.Ok) {
      console.log("NFT listed for sale successfully:", response.Ok);
      await fetchNFTs();
    } else {
      console.error("Error listing NFT for sale:", response.Err);
    }
  }

</script>

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
