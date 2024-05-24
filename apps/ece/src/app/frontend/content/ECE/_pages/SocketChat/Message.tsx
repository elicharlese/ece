import React from 'react';

interface MessageProps {
  id: number;
  content: string;
  onEdit: (id: number, content: string) => void;
  onDelete: (id: number) => void;
}

const Message: React.FC<MessageProps> = ({ id, content, onEdit, onDelete }) => {
  return (
    <div className="message">
      <div>{content}</div>
      <button onClick={() => onEdit(id, content)}>Edit</button>
      <button onClick={() => onDelete(id)}>Delete</button>
    </div>
  );
};

export default Message;