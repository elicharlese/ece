import React, { useEffect } from 'react';
import styles from '../styles/Contribute.module.scss';

const CrowdfundingList: React.FC = () => {
  useEffect(() => {
    // Redirect to NEAR's catalogue of applications
    window.location.href = 'https://dev.near.org/applications';
  }, []);

  return (
    <div className={styles.crowdfundingList}>
      <div className={styles.comingSoonContainer}>
        <h1 className={styles.comingSoonTitle}>Crowdfunding Feature Coming Soon!</h1>
        <p className={styles.comingSoonDescription}>
          Our crowdfunding feature is under development and will be available soon.
        </p>
        <p className={styles.redirectNotice}>
          In the meantime, check out NEAR's catalogue of applications by clicking{' '}
          <a href="https://dev.near.org/applications" className={styles.nearLink}>here</a>.
        </p>
      </div>
    </div>
  );
};

export default CrowdfundingList;