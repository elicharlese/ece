import * as nearAPI from 'near-api-js';

const { connect, keyStores, WalletConnection } = nearAPI;

const CONTRACT_NAME = 'your_contract_name.testnet';

class SocketChatService {
  walletConnection: any;
  contract: any;

  async init() {
    const config = {
      networkId: "testnet",
      keyStore: new keyStoreore(),
      nodeUrl: "https://rpc.testnet.near.org",
      walletUrl: "https://wallet.testnet.near.org",
      helperUrl: "https://helper.testnet.near.org",
    };
    const near = await connect(config);
    this.walletConnection = new WalletConnection(near, null);
    this.contract = new nearAPI.Contract(this.walletConnection.account(), CONTRACT_NAME, {
      viewMethods: ['view_chat'],
      changeMethods: ['handle_chat', /* 'edit_message', 'delete_message' */],
    });
  }

  getMessages = async () => {
    await this.init();
    return await this.contract.view_chat();
  };

  sendMessage = async (message: string) => {
    await this.init();
    await this.contract.handle_chat({ user: this.walletConnection.getAccountId(), message, file_id: null });
  };

  editMessage = async (id: number, content: string) => {
    await this.init();
    // Add the method if exists on your smart contract
    // await this.contract.edit_message({ id, content });
  };

  deleteMessage = async (id: number) => {
    await this.init();
    // Add the method if exists on your smart contract
    // await this.contract.delete_message({ id });
  };
}

export const socketChatService = new SocketChatService();