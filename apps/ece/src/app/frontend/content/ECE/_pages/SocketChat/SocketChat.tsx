import React, { useState, useEffect } from 'react';
import { socketChatService } from './SocketChatService';
import Message from './Message';
import MessageInput from './MessageInput';
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

const SocketChat = () => {
  const [chatMessages, setChatMessages] = useState([]);
  const [newMessage, setNewMessage] = useState('');
  const [editingMessageId, setEditingMessageId] = useState<number | null>(null);
  const [editingContent, setEditingContent] = useState('');
  const [isChatOpen, setIsChatOpen] = useState(true);

  useEffect(() => {
    async function fetchMessages() {
      const messages = await socketChatService.getMessages();
      setChatMessages(messages);
    }
    fetchMessages();

    const handleNewMessage = (message) => {
      setChatMessages((prevMessages) => [...prevMessages, message]);

      if (!isChatOpen) {
        toast.info('New message received');
      }
    };

    socketChatService.on('message', handleNewMessage);

    return () => {
      socketChatService.off('message', handleNewMessage);
    };
  }, [isChatOpen]);

  useEffect(() => {
    const handleConnection = () => {
      socketChatService.connect();

      socketChatService.on('disconnect', () => {
        console.warn('WebSocket disconnected. Attempting to reconnect...');
        setTimeout(() => socketChatService.connect(), 3000);
      });
    };

    handleConnection();

    return () => {
      socketChatService.disconnect();
    };
  }, []);

  const handleSendMessage = async () => {
    if (newMessage) {
      await socketChatService.sendMessage(newMessage);
      setNewMessage('');
      const messages = await socketChatService.getMessages();
      setChatMessages(messages);
    }
  };

  const handleEditMessage = (id: number, content: string) => {
    setEditingMessageId(id);
    setEditingContent(content);
  };

  const handleUpdateMessage = async () => {
    if (editingMessageId !== null) {
      await socketChatService.editMessage(editingMessageId, editingContent);
      setEditingMessageId(null);
      setEditingContent('');
      const messages = await socketChatService.getMessages();
      setChatMessages(messages);
    }
  };

  const handleDeleteMessage = async (id: number) => {
    await socketChatService.deleteMessage(id);
    const messages = await socketChatService.getMessages();
    setChatMessages(messages);
  };

  const toggleChat = () => {
    setIsChatOpen(!isChatOpen);
  };

  return (
    <div className="cde-environment">
      <div className="chat-toggle">
        <button onClick={toggleChat}>
          {isChatOpen ? 'Close Chat' : 'Open Chat'}
        </button>
      </div>
      {isChatOpen && (
        <div className="chat-container">
          {chatMessages.map((msg: any, index) => (
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
      )}
      <ToastContainer />
    </div>
  );
};

export default SocketChat;