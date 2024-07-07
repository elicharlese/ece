  import React, { useState } from 'react';
  import styles from '../styles/Marketplace.module.scss';

  interface Product {
    id: string;
    name: string;
    price: number;
    description: string;
    category: string;
  }

  const products: Product[] = [
    { id: '1', name: 'Product 1', price: 29.99, description: 'Description for product 1', category: 'Category 1' },
    { id: '2', name: 'Product 2', price: 49.99, description: 'Description for product 2', category: 'Category 1' },
    { id: '3', name: 'Product 3', price: 69.99, description: 'Description for product 3', category: 'Category 2' },
  ];

  const Marketplace: React.FC = () => {
    const [selectedCategory, setSelectedCategory] = useState<string | null>(null);

    const filteredProducts = selectedCategory
      ? products.filter(product => product.category === selectedCategory)
      : products;

    return (
      <div className={styles.container}>
        <h1 className={styles.header}>Marketplace</h1>
        <div className={styles.filters}>
          <button onClick={() => setSelectedCategory(null)}>All</button>
          <button onClick={() => setSelectedCategory('Category 1')}>Category 1</button>
          <button onClick={() => setSelectedCategory('Category 2')}>Category 2</button>
        </div>
        <div className={styles.productList}>
          {filteredProducts.map((product) => (
            <div key={product.id} className={styles.productItem}>
              <h2>{product.name}</h2>
              <p>{product.description}</p>
              <p>${product.price.toFixed(2)}</p>
              <button className={styles.detailButton}>View Details</button>
            </div>
          ))}
        </div>
      </div>
    );
  };

  export default Marketplace;