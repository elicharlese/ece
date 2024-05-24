import React, { useEffect, useState } from 'react';
import './TxnTracker.scss';
import { getTransactions, getRecentActivity } from './api';
import Container from '@mui/material/Container';
import Paper from '@mui/material/Paper';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Button from '@mui/material/Button';
import Grid from '@mui/material/Grid';

const NetworkTrafficLayout = ({ toggleAddressView }) => {
  const [transactions, setTransactions] = useState([]);
  const [activities, setActivities] = useState([]);

  useEffect(() => {
    const fetchTransactions = async () => {
      const result = await getTransactions('user_near_address'); // put the actual address here
      setTransactions(result);
    };

    const fetchActivities = async () => {
      const result = await getRecentActivity();
      setActivities(result);
    };

    fetchTransactions();
    fetchActivities();
  }, []);

  return (
    <Container>
      <h2>Network Traffic</h2>
      <Grid container spacing={2}>
        <Grid item xs={12}>
          <TableContainer component={Paper}>
            <Table className="htop-table">
              <TableHead>
                <TableRow>
                  <TableCell>Process</TableCell>
                  <TableCell>ID</TableCell>
                  <TableCell>Data</TableCell>
                  <TableCell>Timestamp</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {transactions.map((tx, index) => (
                  <TableRow key={index}>
                    <TableCell>Transaction {index + 1}</TableCell>
                    <TableCell>{tx[0]}</TableCell>
                    <TableCell>{tx[1]}</TableCell>
                    <TableCell>{new Date().toLocaleString()}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </Grid>
      </Grid>
      <h2>Recent Activities</h2>
      <Grid container spacing={2}>
        <Grid item xs={12}>
          <TableContainer component={Paper}>
            <Table className="htop-table">
              <TableHead>
                <TableRow>
                  <TableCell>Address</TableCell>
                  <TableCell>Transaction Count</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {activities.map((activity, index) => (
                  <TableRow key={index} onClick={() => toggleAddressView(activity[0])}>
                    <TableCell>{activity[0]}</TableCell>
                    <TableCell>{activity[1]}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </Grid>
      </Grid>
    </Container>
  );
};

const AddressInfoLayout = ({ address, backToNetworkTraffic }) => {
  const [transactions, setTransactions] = useState([]);

  useEffect(() => {
    const fetchTransactions = async () => {
      const result = await getTransactions(address);
      setTransactions(result);
    };

    fetchTransactions();
  }, [address]);

  return (
    <Container>
      <Button variant="contained" onClick={backToNetworkTraffic}>Back</Button>
      <h2>Address: {address}</h2>
      <div className="transactions">
        <h2>Transactions</h2>
        <Grid container spacing={2}>
          {transactions.map((tx, index) => (
            <Grid item xs={12} key={index}>
              <Paper className="transaction">
                <p>ID: {tx[0]}</p>
                <p>Data: {tx[1]}</p>
                <p>Timestamp: {new Date().toLocaleString()}</p>
              </Paper>
            </Grid>
          ))}
        </Grid>
      </div>
    </Container>
  );
};

const TxnTracker = () => {
  const [view, setView] = useState('network');
  const [selectedAddress, setSelectedAddress] = useState(null);

  const toggleAddressView = (address) => {
    setSelectedAddress(address);
    setView('address');
  };

  const backToNetworkTraffic = () => {
    setSelectedAddress(null);
    setView('network');
  };

  return (
    <div>
      {view === 'network' ? (
        <NetworkTrafficLayout toggleAddressView={toggleAddressView} />
      ) : (
        <AddressInfoLayout address={selectedAddress} backToNetworkTraffic={backToNetworkTraffic} />
      )}
    </div>
  );
};

export default TxnTracker;