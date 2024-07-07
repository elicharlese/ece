import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useSelector, useDispatch } from 'react-redux';
import { setUser, setWifiNetworks, setConnectedNetwork } from '../../redux/userSlice';
import { fetchLocalNetworks, connectToNetwork } from './wifiHelper';
import UserCard from '../../components/UserCard';
import WifiNetworksList from '../../components/WifiNetworksList';
import './ThePublic.scss';

const ThePublic: React.FC = () => {
  const [username, setUsername] = useState('');
  const [accountId, setAccountId] = useState('');
  const user = useSelector((state: any) => state.user);
  const wifiNetworks = useSelector((state: any) => state.wifiNetworks);
  const dispatch = useDispatch();

  // Fetch local WiFi networks when user is set
  useEffect(() => {
    if (user.accountId) {
      getLocalWifiNetworks();
    }
  }, [user]);

  // Register user
  const registerUser = async () => {
    try {
      const response = await axios.post('/api/register', { accountId, username });
      dispatch(setUser(response.data));
    } catch (error) {
      console.error('Registration failed', error);
    }
  };

  // Get user details
  const getUser = async () => {
    try {
      const response = await axios.get(`/api/user/${accountId}`);
      dispatch(setUser(response.data));
    } catch (error) {
      console.error('Fetching user failed', error);
    }
  };

  // Get list of local WiFi networks
  const getLocalWifiNetworks = async () => {
    try {
      const networks = await fetchLocalNetworks();
      dispatch(setWifiNetworks(networks));
    } catch (error) {
      console.error('Fetching local WiFi networks failed', error);
    }
  };

  // Connect to selected network
  const handleConnectToNetwork = async (network: string) => {
    try {
      const response = await connectToNetwork(network, user);
      if (response.success) {
        dispatch(setConnectedNetwork(network));
      }
    } catch (error) {
      console.error('Connecting to network failed', error);
    }
  };

  return (
    <div className="the-public flex-col items-center">
      <div className="m-4">
        <h1 className="text-4xl">Public Wifi Sharing</h1>
        <input
          type="text"
          placeholder="Account ID"
          value={accountId}
          onChange={(e) => setAccountId(e.target.value)}
          className="border p-2 m-2"
        />
        <input
          type="text"
          placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          className="border p-2 m-2"
        />
        <button onClick={registerUser} className="bg-blue-500 text-white p-2">Register</button>
        <button onClick={getUser} className="bg-green-500 text-white p-2">Get User</button>
      </div>
      {user && (
        <UserCard user={user} />
      )}
      {wifiNetworks.length > 0 && (
        <WifiNetworksList networks={wifiNetworks} onConnect={handleConnectToNetwork} />
      )}
    </div>
  );
};

export default ThePublic;