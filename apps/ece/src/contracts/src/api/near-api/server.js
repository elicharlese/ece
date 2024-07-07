const express = require('express');
const bodyParser = require('body-parser');
const axios = require('axios');
const userController = require('./controllers/userController');
const cors = require('cors');
const helmet = require('helmet');
const { exec } = require('child_process');

const app = express();
app.use(cors());
app.use(helmet());
app.use(bodyParser.json());

// Improve server capacity for handling concurrent requests
const cluster = require('cluster');
const os = require('os');
const numCPUs = os.cpus().length;

if (cluster.isMaster) {
  console.log(`Master ${process.pid} is running`);

  // Fork workers.
  for (let i = 0; i < numCPUs; i++) {
    cluster.fork();
  }

  cluster.on('exit', (worker, code, signal) => {
    console.log(`Worker ${worker.process.pid} died`);
    console.log('Starting a new worker');
    cluster.fork();
  });
} else {
  // API Endpoints
  app.post('/api/nodes/start', async (req, res) => {
    const { accountId } = req.body;
    try {
      await axios.post('http://orchestration-tool/start-node', { accountId });
      res.status(200).send({ message: 'Node started successfully.' });
    } catch (error) {
      res.status(500).send({ error: 'Failed to start node.' });
    }
  });

  app.post('/api/nodes/stop', async (req, res) => {
    const { accountId } = req.body;
    try {
      await axios.post('http://orchestration-tool/stop-node', { accountId });
      res.status(200).send({ message: 'Node stopped successfully.' });
    } catch (error) {
      res.status(500).send({ error: 'Failed to stop node.' });
    }
  });

  app.get('/api/nodes/:accountId/logs', async (req, res) => {
    const { accountId } = req.params;
    try {
      const response = await axios.get(`http://orchestration-tool/${accountId}/logs`);
      res.status(200).send({ logs: response.data.logs });
    } catch (error) {
      res.status(500).send({ error: 'Failed to fetch logs.' });
    }
  });

  // New API endpoint to execute shell commands
  app.post('/api/nodes/executeCommand', async (req, res) => {
    const { command } = req.body;
    exec(command, (error, stdout, stderr) => {
      if (error) {
        res.status(500).send({ error: error.message });
        return;
      }
      res.send({ output: stdout || stderr });
    });
  });

  // New API endpoint to fetch logs
  app.get('/api/nodes/:accountId/getLogs', async (req, res) => {
    const { accountId } = req.params;
    try {
      // Example logic to fetch logs; replace with actual log fetching logic if necessary
      const logs = 'Sample logs...';
      res.status(200).send(logs);
    } catch (error) {
      res.status(500).send({ error: 'Failed to fetch logs.' });
    }
  });

  app.post('/api/register', userController.registerUser);
  app.get('/api/user/:accountId', userController.getUser);

  const PORT = process.env.PORT || 3000;
  app.listen(PORT, () => console.log(`Worker ${process.pid} running on port ${PORT}`));
}