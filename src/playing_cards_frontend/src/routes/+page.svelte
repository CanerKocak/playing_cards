<script>
  import "../index.scss";
  import { backend } from "$lib/canisters";
  import { onMount } from "svelte";

  let greeting = "";
  let nfts = [];
  let mintPrincipal = "";
  let mintMetadata = [];
  let mintContent = new Uint8Array();

  onMount(fetchNFTs);

  async function onSubmit(event) {
    const name = event.target.name.value;
    greeting = await backend.greet(name);
  }

  async function fetchNFTs() {
    nfts = await backend.listAllNftsFull();
  }

  async function mintNFT() {
    const response = await backend.mintDip721(mintPrincipal, mintMetadata, mintContent);
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
</script>

<main>
  <img src="/logo2.svg" alt="DFINITY logo" />
  <h1>Welcome to Playing Cards Terminal</h1>

  <form on:submit|preventDefault={onSubmit}>
    <label for="name">Enter your name:</label>
    <input type="text" id="name" name="name" required />
    <button type="submit">Submit</button>
  </form>

  {#if greeting}
    <p>Greeting: {greeting}</p>
  {/if}

  <h2>NFTs</h2>
  <button on:click={fetchNFTs}>Refresh NFTs</button>

  {#if nfts.length > 0}
    <ul>
      {#each nfts as nft}
        <li>
          <p>ID: {nft.id}</p>
          <p>Owner: {nft.owner}</p>
          <!-- Display other NFT properties as needed -->
        </li>
      {/each}
    </ul>
  {:else}
    <p>No NFTs found.</p>
  {/if}

  <h2>Mint NFT</h2>
  <label for="mintPrincipal">Mint to Principal:</label>
  <input type="text" id="mintPrincipal" bind:value={mintPrincipal} />

  <label for="mintMetadata">Metadata:</label>
  <textarea id="mintMetadata" bind:value={mintMetadata}></textarea>

  <label for="mintContent">Content:</label>
  <input type="file" id="mintContent" on:change={handleFileUpload} />

  <button on:click={mintNFT}>Mint NFT</button>
</main>