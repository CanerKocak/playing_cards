<script>
  import card from "../../../../hearts 4.png";
</script>

<div class="container">
  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">NFT Collection Overview</div>
    </div>
    <div class="window-body">
        <li><b>Wow, such innovation!</b> Presenting the <b>Playing Cards NFT Collection</b>, an amazing project bringing the classic card game to the Windoge98 on-chain operating system.</li>
        <li>This awesome collection has a full deck of 54 digital playing cards, each one a unique non-fungible token (NFT) on the blockchain. <b>Much wow!</b></li>
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Features</div>
    </div>
    <div class="window-body">
      <li><b>NFT Playing Cards:</b> Each card minted as a unique NFT. So unique, very ownable.</li>
      <li><b>Customizable Design:</b> Personalize your cards with custom images. Such style!</li>
      <li><b>In-Game Integration:</b> Use cards for all card games in Windoge98. Much play!</li>
      <li><b>Exclusive Advertising Opportunity:</b> Turn cards into ad billboards. Such marketing!</li>
      <li><b>Windoge98 Desktop Integration:</b> Easy management on Windoge98 desktop. Much convenient!</li>
      <li><b>Built-in Marketplace:</b> Buy, sell, or trade NFT playing cards. Such trade!</li>
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Getting Started</div>
    </div>
    <div class="window-body">
      <li>Visit the Windoge98 desktop at <a href="http://desktop.windoge98.com" target="_blank"><b>desktop.windoge98.com</b></a>. Much site!</li>
      <li>Connect with your Internet Identity and ensure you have enough EXE tokens. <b>Such connect!</b></li>
      <li>Join the exclusive initial sale of the Playing Cards NFTs. <b>Wow, so early!</b></li>
      <li>Customize your card designs and play various card games. <b>Much fun!</b></li>
      <li>Use the built-in marketplace to manage your collection. <b>So easy!</b></li>
    </div>
  </div>

  <div class="window">
    <div class="title-bar">
      <div class="title-bar-text">Customization</div>
    </div>
    <div class="window-body">
      <li>Customize your card designs by burning EXE. <b>So cool!</b></li>
      <li>Personalize your cards with whatever you like. <b>So creative!</b></li>
      <img src={card} alt="Custom Card Example" style="max-width: 100%; height: auto; margin-top: 10px;">
    </div>
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
    margin: 20px;
  }

  .window {
    border: 1px solid #ccc;
    border-radius: 5px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    height: 100%;
    width: 300px;
  }

  .title-bar {
    background-color: #f0f0f0;
    padding: 10px;
    border-bottom: 1px solid #ccc;
  }

  .title-bar-text {
    font-weight: bold;
    font-size: 16px;
  }

  .window-body {
    padding: 15px;
    font-size: 14px;
    line-height: 1.6;
  }

  ul {
    list-style-type: disc;
    margin-left: 20px;
    padding-left: 20px;
  }

  li {
    margin-bottom: 10px;
  }

  a {
    font-weight: bold;
    text-decoration: none;
    color: #0066cc;
  }

  img {
    max-width: 100%; 
    height: auto;
    margin-top: 10px;
  }
</style>
