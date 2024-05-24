const nearAPI = require('near-api-js');
const { connect, keyStores, WalletConnection } = nearAPI;

const nearConfig = {
    networkId: 'default',
    keyStore: new keyStoreore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
};

async function initNear() {
    const near = await connect(nearConfig);
    const wallet = new WalletConnection(near);
    return wallet;
}

// Call set_settings method
async function setSettings(accountId, version, strategy, strategyFile, network) {
    const wallet = await initNear();
    const account = wallet.account();
    
    await account.functionCall({
        contractId: 'your_contract_account_id.testnet', // Replace with your contract ID
        methodName: 'set_settings',
        args: { account_id: accountId, version, strategy, strategy_file: strategyFile, network },
    });
}

// Call get_settings method
async function getSettings(accountId) {
    const wallet = await initNear();
    const account = wallet.account();
    
    const settings = await account.viewFunction(
        'your_contract_account_id.testnet', // Replace with your contract ID
        'get_settings',
        { account_id: accountId }
    );
    console.log(settings);
}

// Usage examples
setSettings('example.testnet', '1.0.0', 'strategy1', 'file1', 'mainnet');
getSettings('example.testnet');