import React from 'react';
import './UserCard.scss';

interface UserCardProps {
  user: {
    accountId: string;
    username: string;
    connectedDevices: string;
    connectedNetwork: string;
  };
}

const UserCard: React.FC<UserCardProps> = ({ user }) => {
  return (
    <div className="user-card m-4 p-4 border">
      <h2 className="text-2xl">User Details</h2>
      <p><strong>Account ID:</strong> {user.accountId}</p>
      <p><strong>Username:</strong> {user.username}</p>
      <p><strong>Connected Devices:</strong> {user.connectedDevices}</p>
      <p><strong>Connected Network:</strong> {user.connectedNetwork || 'None'}</p>
    </div>
  );
};

export default UserCard;