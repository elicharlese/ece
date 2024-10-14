import React, { useState } from 'react';
import './IDW.scss';

const IDW = () => {
  const [agentConfig, setAgentConfig] = useState({});
  const [uploadedTemplate, setUploadedTemplate] = useState(null);
  const [testResult, setTestResult] = useState(null);

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
      // Simulate template upload
      setUploadedTemplate(file.name);
    }
  };

  const handleTestAgent = () => {
    // Simulate testing the agent
    setTestResult('Test successful!');
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
        {/* Placeholder for marketplace integration */}
      </div>
    </div>
  );
};

export default IDW;