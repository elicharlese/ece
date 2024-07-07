import React from 'react';
import styles from '../styles/Wallet.module.scss';

const Wallet: React.FC = () => {
  return (
    <div className={styles.container}>
      <h1 className={styles.header}>Wallet</h1>
      <div className={styles.walletInfo}>
        <div className={styles.balance}>
          <h2>Balance</h2>
          <p>$123.45</p>
        </div>
        <div className={styles.transactions}>
          <h2>Recent Transactions</h2>
          <ul>
            <li>Transaction 1 - $20.00</li>
            <li>Transaction 2 - $15.00</li>
            <li>Transaction 3 - $30.00</li>
          </ul>
        </div>
      </div>
      <div className={styles.walletActions}>
        <button className={styles.actionButton}>Add Funds</button>
        <button className={styles.actionButton}>Withdraw Funds</button>
      </div>
    </div>
  );
};

export default Wallet;