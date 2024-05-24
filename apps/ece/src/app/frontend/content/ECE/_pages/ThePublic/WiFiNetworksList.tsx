import React from 'react';
import './WifiNetworksList.scss';

interface WifiNetworksListProps {
  networks: string[];
  onConnect: (network: string) => void;
}

const WifiNetworksList: React.FC<WifiNetworksListProps> = ({ networks, onConnect }) => {
  return (
    <div className="wifi-networks-list m-4 p-4 border">
      <h2 className="text-2xl">Available WiFi Networks</h2>
      <ul>
        {networks.map((network) => (
          <li key={network}>
            {network}
            <button onClick={() => onConnect(network)} className="bg-yellow-500 text-white p-2 ml-2">Connect</button>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default WifiNetworksList;