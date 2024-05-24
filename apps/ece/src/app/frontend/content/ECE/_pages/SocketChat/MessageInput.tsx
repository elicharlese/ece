import React from 'react';

interface MessageInputProps {
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: () => void;
  isEditing: boolean;
}

const MessageInput: React.FC<MessageInputProps> = ({ value, onChange, onSubmit, isEditing }) => {
  return (
    <div>
      <input 
        type="text" 
        value={value}
        onChange={onChange}
      />
      <button onClick={onSubmit}>{isEditing ? 'Update' : 'Send'}</button>
    </div>
  );
};

export default MessageInput;