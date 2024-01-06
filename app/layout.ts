'use client';
import './globals.css';
import '@near-wallet-selector/modal-ui/styles.css';

import { NetworkId } from './config';
<<<<<<< HEAD
import { Navigation } from './frontend/navigation';
=======
import { Navigation } from './components/navigation';
>>>>>>> 18e5a849 (Scaffold basic file structure)
import { useInitWallet } from './wallets/wallet-selector';

export default function RootLayout({ children }) {

  useInitWallet({ createAccessKeyFor: '', networkId: NetworkId });

  return (
    <html lang="en">
      <body>
        <Navigation />
        {children}
      </body>
    </html>
  );
}
