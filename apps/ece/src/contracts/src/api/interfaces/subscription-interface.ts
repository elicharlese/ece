import { connect, Contract, keyStores, WalletConnection } from 'near-api-js';

const config = {
  networkId: 'testnet',
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  explorerUrl: 'https://explorer.testnet.near.org',
};

export class SubscriptionInterface {
  private contract: any;

  constructor() {
    this.initContract();
  }

  private async initContract() {
    const near = await connect(config);
    const wallet = new WalletConnection(near);
    const account = wallet.account();
    this.contract = new Contract(account, 'subscription.testnet', {
      viewMethods: ['get_subscription', 'get_token_balance', 'get_conversion_rate'],
      changeMethods: ['subscribe', 'purchase_tokens'],
    });
  }

  async subscribe(accountId: string, tier: string) {
    await this.contract.subscribe({ account_id: accountId, tier });
  }

  async purchaseTokens(accountId: string, amount: number) {
    await this.contract.purchase_tokens({ account_id: accountId, amount });
  }

  async getSubscription(accountId: string) {
    return await this.contract.get_subscription({ account_id: accountId });
  }

  async getTokenBalance(accountId: string) {
    return await this.contract.get_token_balance({ account_id: accountId });
  }

  async getConversionRate() {
    return await this.contract.get_conversion_rate();
  }
}