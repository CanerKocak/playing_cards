<script>
  export let nft;
  import logo from "../../../../../logo.png";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  function handleSell() {
    dispatch("sell", { nft });
  }
</script>

<div class="card" style="width: 18rem;">
  <section class="p-4 flex justify-center">
    <img src={logo} alt={nft.name} />
  </section>
  <header class="card-header">{nft.name}</header>
  <footer class="card-footer">{nft.description}</footer>
  <div class="card-body">
    <button class="btn btn-primary" on:click={handleSell}>List for Sale</button>
  </div>
</div>