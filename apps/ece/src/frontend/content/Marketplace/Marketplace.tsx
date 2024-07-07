import React from 'react';
import Home from './_pages/Home';
import MarketplacePage from './_pages/Marketplace';
import Wallet from './_pages/Wallet';
import styles from './styles/Marketplace.module.scss';

const Marketplace: React.FC = () => {
  return (
    <div className={styles.marketplaceContainer}>
      <nav className={styles.navbar}>
        <ul className={styles.navList}>
          <li className={styles.navItem}><a href="#home">Home</a></li>
          <li className={styles.navItem}><a href="#marketplace">Marketplace</a></li>
          <li className={styles.navItem}><a href="#wallet">Wallet</a></li>
        </ul>
      </nav>
      <section id="home">
        <Home />
      </section>
      <section id="marketplace">
        <MarketplacePage />
      </section>
      <section id="wallet">
        <Wallet />
      </section>
    </div>
  );
};

export default Marketplace;