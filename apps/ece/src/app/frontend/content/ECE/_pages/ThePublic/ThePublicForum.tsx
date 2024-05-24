import React, { useState, useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import axios from 'axios';

const ThePublicForum: React.FC = () => {
  const [newTopic, setNewTopic] = useState('');
  const [topics, setTopics] = useState([]);
  const user = useSelector((state: any) => state.user);
  const dispatch = useDispatch();

  useEffect(() => {
    loadTopics();
  }, []);

  const loadTopics = async () => {
    try {
      const response = await axios.get('/api/forum/topics');
      setTopics(response.data);
    } catch (error) {
      console.error('Error loading topics:', error);
    }
  };

  const handleNewTopic = async () => {
    try {
      const response = await axios.post('/api/forum/topics', { title: newTopic, accountId: user.accountId });
      setNewTopic('');
      loadTopics();
    } catch (error) {
      console.error('Error creating new topic:', error);
    }
  };

  return (
    <div className="flex flex-col items-center">
      <div className="m-4 w-full md:w-1/2">
        <h1 className="text-4xl">Public Forum</h1>
        <input
          type="text"
          value={newTopic}
          onChange={(e) => setNewTopic(e.target.value)}
          className="border p-2 m-2 w-full"
          placeholder="New Topic Title"
        />
        <button onClick={handleNewTopic} className="bg-blue-500 text-white p-2">Create Topic</button>
        {topics.map((topic: any, index: number) => (
          <div key={index} className="m-4 p-4 border">
            <h2 className="text-2xl">{topic.title}</h2>
            <p><small>Posted by {topic.accountId}</small></p>
            <button className="bg-gray-500 text-white p-2 mt-2">View Thread</button>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ThePublicForum;