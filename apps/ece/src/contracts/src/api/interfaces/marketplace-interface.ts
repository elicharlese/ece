import { connect, Contract, keyStores, WalletConnection } from 'near-api-js';

const config = {
  networkId: 'testnet',
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  explorerUrl: 'https://explorer.testnet.near.org',
};

export class MarketplaceInterface {
  private contract: any;

  constructor() {
    this.initContract();
  }

  private async initContract() {
    const near = await connect(config);
    const wallet = new WalletConnection(near);
    const account = wallet.account();
    this.contract = new Contract(account, 'marketplace.testnet', {
      viewMethods: ['get_product'],
      changeMethods: ['list_product', 'purchase_product'],
    });
  }

  async listProduct(id: number, ownerId: string, metadata: string, price: number) {
    await this.contract.list_product({ id, owner_id: ownerId, metadata, price });
  }

  async purchaseProduct(productId: number) {
    await this.contract.purchase_product({ product_id: productId });
  }

  async getProduct(productId: number) {
    return await this.contract.get_product({ product_id: productId });
  }
}