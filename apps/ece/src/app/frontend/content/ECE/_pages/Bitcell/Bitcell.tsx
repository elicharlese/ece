import React, { useEffect, useState } from 'react';
import Bitcell3DSpline from './Bitcell3DSpline';
import './Bitcell.scss';
import { connect, Contract, keyStores, WalletConnection } from 'near-api-js';

interface PerformanceMetrics {
  profit: number;
  loss: number;
  trades_executed: number;
}

interface HealthStatus {
  organelles: Record<string, { status: boolean, last_check: number }>;
  alerts: string[];
}

const Bitcell = () => {
  const [performanceMetrics, setPerformanceMetrics] = useState<PerformanceMetrics | null>(null);
  const [healthStatus, setHealthStatus] = useState<HealthStatus | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const initContract = async () => {
    const config = {
      networkId: "testnet",
      keyStore: new keyStores.BrowserLocalStorageKeyStore(),
      nodeUrl: "https://rpc.testnet.near.org",
      walletUrl: "https://wallet.testnet.near.org",
      helperUrl: "https://helper.testnet.near.org",
      explorerUrl: "https://explorer.testnet.near.org",
    };
    const near = await connect(config);
    const wallet = new WalletConnection(near);
    const account = wallet.account();
    return new Contract(account, "bitcell.testnet", {
      viewMethods: ["getPerformanceMetrics", "getHealthStatus"],
      changeMethods: ["updateHeartbeat", "checkBollingerBands"],
    });
  };

  const fetchData = async () => {
    try {
      setLoading(true);
      const contract: any = await initContract();
      const fetchedPerformanceMetrics = await contract.getPerformanceMetrics();
      const fetchedHealthStatus = await contract.getHealthStatus();
      setPerformanceMetrics(fetchedPerformanceMetrics);
      setHealthStatus(fetchedHealthStatus);
    } catch (err) {
      setError(err.message ?? 'Failed to fetch data from contract');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const renderLoading = () => <div>Loading...</div>;
  const renderError = () => <div>{error}</div>;

  const renderContent = () => (
    <div className="bitcell-container">
      <div className="bitcell-3d">
        <Bitcell3DSpline 
          performanceMetrics={performanceMetrics} 
          healthStatus={healthStatus} 
        />
      </div>
      <div className="info-box performance">
        <h3 className="info-heading">Performance Metrics</h3>
        <p className="info-text">Profit: {performanceMetrics?.profit}</p>
        <p className="info-text">Loss: {performanceMetrics?.loss}</p>
        <p className="info-text">Trades Executed: {performanceMetrics?.trades_executed}</p>
      </div>
      <div className="info-box health">
        <h3 className="info-heading">Health Status</h3>
        <ul className="info-text">
          {healthStatus && Object.keys(healthStatus.organelles).map((organelle) => (
            <li key={organelle}>
              {organelle}: {healthStatus.organelles[organelle].status ? 'Healthy' : 'Unhealthy'} (Last checked: {new Date(healthStatus.organelles[organelle].last_check * 1000).toLocaleString()})
            </li>
          ))}
        </ul>
      </div>
      <div className="info-box alerts">
        <h3 className="info-heading">Alerts</h3>
        <ul className="info-text">
          {healthStatus?.alerts.map((alert, index) => (
            <li key={index}>{alert}</li>
          ))}
        </ul>
      </div>
    </div>
  );

  if (loading) return renderLoading();
  if (error) return renderError();

  return renderContent();
};

export default Bitcell;