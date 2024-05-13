import * as nearAPI from 'near-api-js';

// Near configuration setup 
const config = {
  networkId: "testnet",
  keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
};

// Assuming you have some kind of smart contract for startups
// Startup Interface
class StartupsInterface {
  async function initialize() {
    this.near = await nearAPI.connect(config);
    this.wallet = new nearAPI.WalletConnection(this.near);
    this.account = this.wallet.account();
  }

  async function getStartups() {
    // Call smart contract method to retrieve startups' details
    return await this.account.viewFunction('startupContractName', 'get_startups');
  }

  async function investInStartup(startupId, amount) {
    // Call smart contract method to invest a certain amount in a startup
    return await this.account.functionCall({
      contractId: 'startupContractName',
      methodName: 'invest_in_startup',
      args: { startup_id: startupId },
      attachedDeposit: amount,
    });
  }

  // ... other startup related methods ...
}

export default StartupsInterface;
