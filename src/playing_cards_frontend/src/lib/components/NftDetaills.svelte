<script>
  import { Principal } from "@dfinity/principal";
  import { createEventDispatcher } from "svelte";
  import { cardBackend, ledgerBackend } from "$lib/canisters/canisters";

  export let nft;
  let newImageData = null;
  const dispatch = createEventDispatcher();

  function handleImageChange(event) {
    const file = event.target.files[0];
    const reader = new FileReader();
    reader.onload = function (e) {
      newImageData = new Uint8Array(e.target.result);
    };
    reader.readAsArrayBuffer(file);
  }

  async function updateImage() {
    await cardBackend.updateNftImage(nft.id, [...newImageData]);
    dispatch("update", { nftId: nft.id });
  }

  function getImageDataUrl(imageData) {
    const base64Data = btoa(
      String.fromCharCode.apply(null, new Uint8Array(imageData))
    );
    return `data:image/png;base64,${base64Data}`;
  }
</script>

<div>
  <h1>{nft.name}</h1>
  <img src={getImageDataUrl(nft.content)} alt={nft.name} />
  <p>Owner: {nft.owner.toText()}</p>
  <div>
    <label for="newImage">Upload New Image:</label>
    <input
      type="file"
      id="newImage"
      accept="image/*"
      on:change={handleImageChange}
    />
    <button on:click={updateImage}>Update Image</button>
  </div>
</div>
