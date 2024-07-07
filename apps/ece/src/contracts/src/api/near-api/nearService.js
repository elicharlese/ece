const nearAPI = require('near-api-js');
const { connect, keyStores, WalletConnection } = nearAPI;

const config = {
  networkId: 'testnet',
  keyStore: new keyStoreore(),
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org'
};

// Initialize connection to NEAR
const near = await connect(config);
const wallet = new WalletConnection(near);

exports.registerUser = async (accountId, username) => {
  // Interact with NEAR smart contract to register the user
};

exports.getUser = async (accountId) => {
  // Interact with NEAR smart contract to fetch user data
};