import { useMetamask } from '@thirdweb-dev/react';

function ConnectWalletButton() {
  const connectWithMetamask = useMetamask();

  return <button onClick={connectWithMetamask}>Connect Wallet</button>;
}