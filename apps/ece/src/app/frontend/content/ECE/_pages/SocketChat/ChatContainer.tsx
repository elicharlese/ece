import React, { useState, useEffect } from 'react';
import { SocketChatService } from './SocketChatService'; // Hypothetical service layer to handle API calls
import Message from './Message';
import MessageInput from './MessageInput';

const ChatContainer = () => {
  const [chatMessages, setChatMessages] = useState([]);
  const [newMessage, setNewMessage] = useState("");
  const [editingMessageId, setEditingMessageId] = useState<number | null>(null);
  const [editingContent, setEditingContent] = useState("");

  useEffect(() => {
    fetchMessages();
  }, []);

  const fetchMessages = async () => {
    const messages = await SocketChatService.getMessages();
    setChatMessages(messages);
  };

  const handleSendMessage = async () => {
    await SocketChatService.sendMessage(newMessage);
    setNewMessage("");
    fetchMessages();
  };

  const handleEditMessage = (id: number, content: string) => {
    setEditingMessageId(id);
    setEditingContent(content);
  };

  const handleUpdateMessage = async () => {
    if (editingMessageId !== null) {
      await SocketChatService.editMessage(editingMessageId, editingContent);
      setEditingMessageId(null);
      setEditingContent("");
      fetchMessages();
    }
  };

  const handleDeleteMessage = async (id: number) => {
    await SocketChatService.deleteMessage(id);
    fetchMessages();
  };

  return (
    <div className="chat-container">
      {chatMessages.map((msg, index) => (
        <Message 
          key={index} 
          id={index} 
          content={msg.content} 
          onEdit={handleEditMessage}
          onDelete={handleDeleteMessage}
        />
      ))}

      <MessageInput 
        value={editingMessageId !== null ? editingContent : newMessage}
        onChange={(e) => editingMessageId !== null ? setEditingContent(e.target.value) : setNewMessage(e.target.value)}
        onSubmit={editingMessageId !== null ? handleUpdateMessage : handleSendMessage}
        isEditing={editingMessageId !== null}
      />
    </div>
  );
};

export default ChatContainer;