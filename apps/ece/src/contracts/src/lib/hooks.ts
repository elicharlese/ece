import { useState, useEffect } from 'react';
import { connect, WalletConnection } from 'near-api-js';
import { getConfig } from './utils';

const nearConfig = getConfig('testnet'); // or "testnet" or "mainnet"

export const useNear = () => {
  const [walletConnection, setWalletConnection] = useState<WalletConnection | null>(null);

  useEffect(() => {
    (async () => {
      const near = await connect(nearConfig);
      const wallet = new WalletConnection(near);
      setWalletConnection(wallet);
    })();
  }, []);

  return { walletConnection };
};