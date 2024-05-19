<script>
  export let nft;
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  function handleRemoveCard() {
    dispatch("remove", { nft });
  }

  function getImageDataUrl(imageData) {
    const base64Data = btoa(String.fromCharCode.apply(null, new Uint8Array(imageData)));
    return `data:image/png;base64,${base64Data}`;
  }
</script>

<div class="window" style="width: 300px">
  <div class="title-bar">
    <div class="title-bar-text">A Window With Stuff In It</div>
    <div class="title-bar-controls">
      <button aria-label="Close" on:click={handleRemoveCard}></button>
    </div>
  </div>
  <div class="window-body bg-c3c3c3">
    <div class="nft-card">
      <div class="card" style="width: 18rem;">
        <img src={getImageDataUrl(nft.content)} alt={nft.name} />
        <div class="card-body">
          <button>Buy</button>
          <button>View</button>
          <button>Sell</button>
        </div>
      </div>
    </div>
  </div>
</div>