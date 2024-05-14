import { useMetamask, useAddress, ChainId, ThirdwebProvider } from '@thirdweb-dev/react';

const ConnectWallet = () => {
  const connectWithMetamask = useMetamask();
  const address = useAddress();

  return (
    <div>
      {address ? (
        <p>Wallet Connected: {address}</p>
      ) : (
        <button onClick={connectWithMetamask}>Connect Wallet</button>
      )}
    </div>
  );
};

export const WalletProvider = ({ children }) => (
  <ThirdwebProvider desiredChainId={ChainId.Mainnet}>
    <ConnectWallet />
    {children}
  </ThirdwebProvider>
);