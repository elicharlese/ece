import React from 'react';
import { Box } from '@mui/material';
import Contribute from '../content/Contribute/Contribute';
import ECE from '../content/ECE/ECE';
import Marketplace from '../content/Marketplace/Marketplace';
import styles from './styles/app.module.scss';

const BlendedMode: React.FC = () => (
  <Box className={`${styles.appContainer} ${styles.blendedMode} flex flex-col items-center justify-center w-full h-screen bg-gray-100`}>
    <h1 className={styles.appHeader}>Blended Mode</h1>
    <div className={styles.appContent}>
      <Contribute />
      <ECE />
      <Marketplace />
    </div>
  </Box>
);

export default BlendedMode;