// node-management-api.js
import axios from 'axios';

const API_URL = 'http://your-backend-server/api/nodes'; // Replace with your actual backend server URL

export const startNode = async (accountId) => {
    await axios.post(`${API_URL}/start`, { accountId });
};

export const stopNode = async (accountId) => {
    await axios.post(`${API_URL}/stop`, { accountId });
};

export const getNodeLogs = async (accountId) => {
    const response = await axios.get(`${API_URL}/${accountId}/logs`);
    return response.data.logs;
};