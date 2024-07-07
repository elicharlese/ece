import React from 'react';
import { BrowserRouter as Router, Route, Switch, Link } from 'react-router-dom';
import Bitcell from './_pages/Bitcell';
import ThePublic from './_pages/ThePublic';
import SocketChat from './_pages/SocketChat';
import NanoNode from './_pages/NanoNode';
import TxnTracker from './_pages/TxnTracker';
import styles from './styles/ECE.module.scss';

const ECE: React.FC = () => {
  return (
    <Router>
      <div className={styles.container}>
        <nav className={styles.nav}>
          <ul className={styles.navList}>
            <li className={styles.navItem}><Link to="/bitcell" className={styles.navLink}>Bitcell</Link></li>
            <li className={styles.navItem}><Link to="/the-public" className={styles.navLink}>The Public</Link></li>
            <li className={styles.navItem}><Link to="/socket-chat" className={styles.navLink}>Socket Chat</Link></li>
            <li className={styles.navItem}><Link to="/nano-node" className={styles.navLink}>Nano Node</Link></li>
            <li className={styles.navItem}><Link to="/txn-tracker" className={styles.navLink}>Txn Tracker</Link></li>
          </ul>
        </nav>
        <div className={styles.content}>
          <Switch>
            <Route path="/bitcell" component={Bitcell} />
            <Route path="/the-public" component={ThePublic} />
            <Route path="/socket-chat" component={SocketChat} />
            <Route path="/nano-node" component={NanoNode} />
            <Route path="/txn-tracker" component={TxnTracker} />
            <Route path="/" exact component={() => <div>Welcome to ECE</div>} />
          </Switch>
        </div>
      </div>
    </Router>
  );
};

export default ECE;