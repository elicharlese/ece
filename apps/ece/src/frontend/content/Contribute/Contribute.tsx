import React from 'react';
import styles from './Contribute.module.scss';
import Startups from './Startups';
import Crowdfunding from './Crowdfunding';
import { useTranslation } from 'react-i18next';
import { useRouter } from 'next/router';
import { useStore } from 'react-redux';
import { useDispatch } from 'react-redux';
import { setCurrentPage } from '../../../store/slices/
import GrantsFilter from '../../Utilities/GrantsFilter';

const Contribute: React.FC = () => {
  return (
    <div className={styles.container}>
      <h1 className={styles.header}>Contribute</h1>
      <GrantsFilter />
      <div className={styles.content}>
        <div className={styles.section}>
          <Startups />
        </div>
        <div className={styles.section}>
          <Crowdfunding />
        </div>
      </div>
    </div>
  );
};

export default Contribute;