import React, { useState } from 'react';
import './IDW.scss';

interface Configuration {
  id: string;
  name: string;
  price: number;
  description: string;
}

const availableConfigurations: Configuration[] = [
  { id: '1', name: 'Basic Automation', price: 19.99, description: 'Automate basic tasks with ease.' },
  { id: '2', name: 'Advanced Analytics', price: 49.99, description: 'Gain insights with advanced analytics.' },
  { id: '3', name: 'Full Suite', price: 99.99, description: 'Unlock all features and capabilities.' },
];

const IDW = () => {
  const [agentConfig, setAgentConfig] = useState({});
  const [uploadedTemplate, setUploadedTemplate] = useState(null);
  const [testResult, setTestResult] = useState(null);
  const [purchasedConfigurations, setPurchasedConfigurations] = useState<Configuration[]>([]);

  const handleConfigChange = (e) => {
    const { name, value } = e.target;
    setAgentConfig((prevConfig) => ({
      ...prevConfig,
      [name]: value,
    }));
  };

  const handleTemplateUpload = (e) => {
    const file = e.target.files[0];
    if (file) {
      setUploadedTemplate(file.name);
    }
  };

  const handleTestAgent = () => {
    setTestResult('Test successful!');
  };

  const handlePurchase = (config: Configuration) => {
    setPurchasedConfigurations((prevConfigs) => [...prevConfigs, config]);
  };

  return (
    <div className="idw-container">
      <h1>Intelligent Digital Worker Configuration</h1>
      <div className="config-section">
        <h2>Configure Agent</h2>
        <input
          type="text"
          name="agentName"
          placeholder="Agent Name"
          onChange={handleConfigChange}
        />
        <input
          type="text"
          name="taskDescription"
          placeholder="Task Description"
          onChange={handleConfigChange}
        />
        <button onClick={handleTestAgent}>Test Agent</button>
        {testResult && <p>{testResult}</p>}
      </div>
      <div className="template-section">
        <h2>Upload Template</h2>
        <input type="file" onChange={handleTemplateUpload} />
        {uploadedTemplate && <p>Uploaded: {uploadedTemplate}</p>}
      </div>
      <div className="marketplace-section">
        <h2>Marketplace</h2>
        <p>Purchase templates to enhance your IDW capabilities.</p>
        <div className="configurations">
          {availableConfigurations.map((config) => (
            <div key={config.id} className="configuration-item">
              <h3>{config.name}</h3>
              <p>{config.description}</p>
              <p>${config.price.toFixed(2)}</p>
              <button onClick={() => handlePurchase(config)}>Purchase</button>
            </div>
          ))}
        </div>
        <div className="purchased-configurations">
          <h2>Purchased Configurations</h2>
          {purchasedConfigurations.map((config) => (
            <div key={config.id}>
              <h3>{config.name}</h3>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default IDW;