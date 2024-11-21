import axios from 'axios';

const API_URL = 'http://your-backend-server/api/nodes'; // Replace with your actual backend server URL

export class NodeManagementInterface {
  async startNode(accountId: string): Promise<void> {
    try {
      await axios.post(`${API_URL}/start`, { accountId });
      console.log(`Node for account ${accountId} started successfully.`);
    } catch (error) {
      console.error(`Failed to start node for account ${accountId}:`, error);
    }
  }

  async stopNode(accountId: string): Promise<void> {
    try {
      await axios.post(`${API_URL}/stop`, { accountId });
      console.log(`Node for account ${accountId} stopped successfully.`);
    } catch (error) {
      console.error(`Failed to stop node for account ${accountId}:`, error);
    }
  }

  async getNodeLogs(accountId: string): Promise<string[]> {
    try {
      const response = await axios.get(`${API_URL}/${accountId}/logs`);
      console.log(`Logs for account ${accountId} fetched successfully.`);
      return response.data.logs;
    } catch (error) {
      console.error(`Failed to fetch logs for account ${accountId}:`, error);
      return [];
    }
  }
}