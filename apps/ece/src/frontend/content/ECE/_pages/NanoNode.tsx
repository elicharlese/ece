import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './NanoNode.scss';
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

const NanoNode = () => {
    const [accountId] = useState<string>('example.testnet'); // Replace with dynamic accountId if needed
    const [nodeStatus, setNodeStatus] = useState<'stopped' | 'running'>('stopped');
    const [nodeLogs, setNodeLogs] = useState<string>('');
    const [shellInput, setShellInput] = useState<string>('');

    const handleStartNode = async () => {
        try {
            await axios.post('/startNode', { accountId });
            setNodeStatus('running');
            toast.success("Node started successfully");
        } catch (error) {
            toast.error("Failed to start node");
            console.error('Starting node failed', error);
        }
    };

    const handleStopNode = async () => {
        const confirmStop = window.confirm("Are you sure you want to stop the node?");
        if (!confirmStop) return;

        try {
            await axios.post('/stopNode', { accountId });
            setNodeStatus('stopped');
            toast.success("Node stopped successfully");
        } catch (error) {
            toast.error("Failed to stop node");
            console.error('Stopping node failed', error);
        }
    };

    const handleShellInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        setShellInput(e.target.value);
    };

    const handleShellCommand = async () => {
        try {
            const response = await axios.post('/executeCommand', { command: shellInput });
            setNodeLogs(prevLogs => `${prevLogs}\n${response.data.output}`);
            setShellInput('');
        } catch (error) {
            toast.error("Failed to execute command");
            console.error('Executing command failed', error);
        }
    };

    useEffect(() => {
        if (nodeStatus === 'running') {
            const fetchLogs = async () => {
                try {
                    const response = await axios.get('/getLogs');
                    setNodeLogs(response.data);
                } catch (error) {
                    toast.error("Failed to fetch logs");
                    console.error('Fetching logs failed', error);
                }
            };

            const intervalId = setInterval(fetchLogs, 5000); // Fetch logs every 5 seconds
            return () => clearInterval(intervalId);
        }
    }, [nodeStatus]);

    return (
        <div className="nano-node-container">
            <div className="header">
                <div className="title">Nano Node</div>
                <span className="description">Manage Your Node Below</span>
            </div>
            <div className="buttons">
                <button onClick={handleStartNode} disabled={nodeStatus === 'running'}>Start Node</button>
                <button onClick={handleStopNode} disabled={nodeStatus === 'stopped'}>Stop Node</button>
            </div>
            <div className="terminal-container">
                <div className="input-shell">
                    <h2 className="font-bold">Input Shell</h2>
                    <textarea
                        className="shell-textarea"
                        value={shellInput}
                        onChange={handleShellInputChange}
                    />
                    <button onClick={handleShellCommand} className="submit-button">Submit Command</button>
                </div>
                <div className="output-log">
                    <h2 className="font-bold">Node Logs</h2>
                    {nodeLogs && <div><pre>{nodeLogs}</pre></div>}
                </div>
            </div>
            <ToastContainer />
        </div>
    );
};

export default NanoNode;