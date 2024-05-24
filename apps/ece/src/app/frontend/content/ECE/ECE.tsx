// apps/ece/src/app/frontend/content/ECE/ECE.tsx

import React from 'react';
import { BrowserRouter as Router, Route, Switch, Link } from 'react-router-dom';
import Bitcell from './_pages/Bitcell/Bitcell';
import ThePublic from './_pages/ThePublic/ThePublic';
import SocketChat from './_pages/SocketChat/SocketChat';
import NanoNode from './_pages/NanoNode/NanoNode';
import TxnTracker from './_pages/TxnTracker/TxnTracker';

const ECE = () => {
  return (
    <Router>
      <div style={{ padding: '20px' }}>
        <nav>
          <ul>
            <li><Link to="/bitcell">Bitcell</Link></li>
            <li><Link to="/the-public">The Public</Link></li>
            <li><Link to="/socket-chat">Socket Chat</Link></li>
            <li><Link to="/nano-node">Nano Node</Link></li>
            <li><Link to="/txn-tracker">Txn Tracker</Link></li>
          </ul>
        </nav>
        <Switch>
          <Route path="/bitcell" component={Bitcell} />
          <Route path="/the-public" component={ThePublic} />
          <Route path="/socket-chat" component={SocketChat} />
          <Route path="/nano-node" component={NanoNode} />
          <Route path="/txn-tracker" component={TxnTracker} />
          <Route path="/" component={() => <div>Welcome to ECE</div>} />
        </Switch>
      </div>
    </Router>
  );
};

export default ECE;