// Include the 'alchemy-sdk'
const Alchemy = require('alchemy-sdk');

const config = {
  apiKey: "YOUR_ALCHEMY_API_KEY", // Replace with your Alchemy API key
  network: "eth-mainnet", // Specify your network
};

const alchemy = new Alchemy(config);

// Example function to get the latest block
async function getLatestBlock() {
  const latestBlock = await alchemy.core.getBlockNumber();
  console.log(`Latest Ethereum Block is ${latestBlock}`);
}

// Example function call
getLatestBlock();

module.exports = alchemy;