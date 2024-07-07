import React from 'react';
import styles from '../styles/Home.module.scss';

const Home: React.FC = () => {
  return (
    <div className={styles.container}>
      <header className={styles.header}>
        <h1 className={styles.title}>Welcome to Our Marketplace</h1>
        <p className={styles.subtitle}>Discover our most popular products and special offers.</p>
      </header>
      <section className={styles.featured}>
        <h2 className={styles.sectionTitle}>Featured Products</h2>
        <div className={styles.productList}>
          {/* Placeholder for featured product components */}
          <div className={styles.productItem}>Product 1</div>
          <div className={styles.productItem}>Product 2</div>
          <div className={styles.productItem}>Product 3</div>
        </div>
      </section>
      <section className={styles.categories}>
        <h2 className={styles.sectionTitle}>Browse by Categories</h2>
        <div className={styles.categoryList}>
          {/* Placeholder for category components */}
          <div className={styles.categoryItem}>Category 1</div>
          <div className={styles.categoryItem}>Category 2</div>
          <div className={styles.categoryItem}>Category 3</div>
        </div>
      </section>
      <section className={styles.promotions}>
        <h2 className={styles.sectionTitle}>Special Promotions</h2>
        <div className={styles.promotionList}>
          {/* Placeholder for promotion components */}
          <div className={styles.promotionItem}>Promotion 1</div>
          <div className={styles.promotionItem}>Promotion 2</div>
          <div className={styles.promotionItem}>Promotion 3</div>
        </div>
      </section>
    </div>
  );
};

export default Home;