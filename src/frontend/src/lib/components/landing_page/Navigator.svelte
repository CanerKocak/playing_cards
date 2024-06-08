<script>
  // You can import any necessary images or icons here
</script>

<div class="navigator">
  <button class="nav-button">Start</button>
  <button class="nav-button">Deck</button>
  <button class="nav-button">Casino</button>
  <button class="nav-button">Market</button>
  <button class="nav-button">Wallet</button>
</div>

<style>
  .navigator {
    position: fixed;
    bottom: 0px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 10px;
    padding: 10px;
    background: linear-gradient(to top, rgba(99,73,26,1) 0%, rgba(255,171,18,1) 100%);
    border-radius: 20px 20px 0 0;
    box-shadow: 4px 4px 10px rgba(0, 0, 0, 0.4);
    border: 2px solid #000;
    border-bottom: none;
  }

  .nav-button {
    background: #c3c3c3;
    border: none;
    padding: 10px 20px;
    border-radius: 5px;
    cursor: pointer;
    font-family: 'Windows 98', sans-serif;
    font-size: 18px;
    font-weight: bold;
    color: #000;
    transition: background 0.3s, transform 0.3s;
    min-width: 100px;
  }

  .nav-button:active {
    background: #a3a3a3;
    transform: scale(1);
  }
</style>
