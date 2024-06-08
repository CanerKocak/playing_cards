<script>
  import cardafter from "../../../../../../hearts 4.png";
  import cardbefore from "../../../../../../image17.png";
  import Navigator from "./Navigator.svelte";
</script>

<div class="containerr">
  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">NFT Collection Overview</div>
      <div class="title-bar-controls">
        <button class="title-bar-button" aria-label="Minimize"></button>
        <button class="title-bar-button" aria-label="Maximize"></button>
        <button class="title-bar-button" aria-label="Close"></button>
      </div>
    </div>
    <div class="window-body">
      <p>
        <strong>Introducing the Playing Cards NFT Collection!</strong> A revolutionary project bringing the classic card game to the Windoge98 on-chain operating system.
      </p>
      <p>
        This incredible collection features a complete deck of 54 digital playing cards, each one a unique non-fungible token (NFT) on the blockchain.
      </p>
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Key Features</div>
      <div class="title-bar-controls">
        <button class="title-bar-button" aria-label="Minimize"></button>
        <button class="title-bar-button" aria-label="Maximize"></button>
        <button class="title-bar-button" aria-label="Close"></button>
      </div>
    </div>
    <div class="window-body">
      <ul>
        <li><strong>Unique NFT Playing Cards:</strong><br>Each card is minted as a distinct NFT, providing true ownership and collectibility.</li>
        <li><strong>Customizable Designs:</strong><br>Personalize your cards with custom images to showcase your style.</li>
        <li><strong>Seamless In-Game Integration:</strong><br>Use your NFT cards for all card games within the Windoge98 ecosystem.</li>
        <li><strong>Exclusive Advertising Opportunities:</strong><br>Transform your cards into ad billboards for maximum exposure.</li>
        <li><strong>Windoge98 Desktop Integration:</strong><br>Effortlessly manage your collection directly from the Windoge98 desktop.</li>
        <li><strong>Built-in Marketplace:</strong><br>Buy, sell, or trade your NFT playing cards with ease.</li>
      </ul>
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Getting Started</div>
      <div class="title-bar-controls">
        <button class="title-bar-button" aria-label="Minimize"></button>
        <button class="title-bar-button" aria-label="Maximize"></button>
        <button class="title-bar-button" aria-label="Close"></button>
      </div>
    </div>
    <div class="window-body">
      <ol>
        <li><strong>Visit the Windoge98 desktop:</strong><br>Go to <a href="http://desktop.windoge98.com" target="_blank">desktop.windoge98.com</a>.</li>
        <li><strong>Connect with your Internet Identity:</strong><br>Ensure you have sufficient EXE tokens.</li>
        <li><strong>Participate in the exclusive initial sale:</strong><br>Purchase the Playing Cards NFTs.</li>
        <li><strong>Customize your card designs:</strong><br>Enjoy various card games within the platform.</li>
        <li><strong>Utilize the built-in marketplace:</strong><br>Effortlessly manage your collection.</li>
      </ol>
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Card Customization</div>
      <div class="title-bar-controls">
        <button class="title-bar-button" aria-label="Minimize"></button>
        <button class="title-bar-button" aria-label="Maximize"></button>
        <button class="title-bar-button" aria-label="Close"></button>
      </div>
    </div>
    <div class="window-body">
      <p>Customize your card designs by burning EXE tokens and unleash your creativity by personalizing your cards with unique images.</p>
      <div class="card-comparison">
        <img src={cardbefore} alt="Before Customization" class="card-image">
        <img src={cardafter} alt="After Customization" class="card-image">
      </div>
    </div>
  </div>
</div>

<Navigator />

<style>
  .containerr {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: 20px;
    padding: 20px;
    font-family: 'Windows 98', sans-serif;
  }

  .window {
    border: 2px solid #000;
    box-shadow: 4px 4px 10px rgba(0, 0, 0, 0.4);
  }

  .title-bar {
    padding: 10px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: rgb(99,73,26);
    background: linear-gradient(113deg, rgba(99,73,26,1) 0%, rgba(255,171,18,1) 87%);
  }

  .title-bar-button {
    background: #c3c3c3;
    border: none;
    width: 20px;
    height: 20px;
    margin-left: 5px;
    cursor: pointer;
  }

  .title-bar-text {
    font-weight: bold;
    color: white;
    font-size: 18px;
  }

  .window-body {
    padding: 20px;
    color: #000;
    font-size: 24px;
    line-height: 1.6;
  }

  a {
    color: #000;
    text-decoration: underline;
  }

  .card-comparison {
    display: flex;
    justify-content: space-around;
    margin-top: 30px;
  }

  .card-image {
    max-width: 45%;
    height: auto;
    border: 4px solid #000;
    border-radius: 10px;
    box-shadow: 2px 2px 6px rgba(0, 0, 0, 0.3);
  }
</style>
