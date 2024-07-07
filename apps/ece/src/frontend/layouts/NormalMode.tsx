import React from 'react';
import { Box } from '@mui/material';
import Contribute from '../content/Contribute/Contribute';
import ECE from '../content/ECE/ECE';
import Marketplace from '../content/Marketplace/Marketplace';
import styles from './styles/app.module.scss';

const NormalMode: React.FC = () => (
  <Box className={`${styles.appContainer} ${styles.normalMode} flex flex-col items-center justify-center w-full min-h-screen bg-gray-100`}>
    <h1 className={styles.appHeader}>Normal Mode</h1>
    <div className={styles.appContent}>
      <Contribute />
      <ECE />
      <Marketplace />
    </div>
  </Box>
);

export default NormalMode;