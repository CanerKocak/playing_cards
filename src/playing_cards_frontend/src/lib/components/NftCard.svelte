<script>
    export let nft;
    import logo from "../../../../../logo.png";
  </script>
  
  <div class="card" style="width: 18rem;">
    <section class="p-4 flex justify-center">
    <img src={logo} alt={nft.name} />
    </section>
    <header class="card-header">{nft.name}</header>
    <footer class="card-footer">{nft.description}</footer>
  </div>
