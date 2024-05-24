import React, { useState, useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import axios from 'axios';

const ThePublicFeed: React.FC = () => {
  const [newPost, setNewPost] = useState('');
  const [feed, setFeed] = useState([]);
  const user = useSelector((state: any) => state.user);
  const dispatch = useDispatch();

  useEffect(() => {
    loadFeed();
  }, []);

  const loadFeed = async () => {
    try {
      const response = await axios.get('/api/feed');
      setFeed(response.data);
    } catch (error) {
      console.error('Error loading feed:', error);
    }
  };

  const handleNewPost = async () => {
    try {
      const response = await axios.post('/api/feed', { content: newPost, accountId: user.accountId });
      setNewPost('');
      loadFeed();
    } catch (error) {
      console.error('Error creating new post:', error);
    }
  };

  return (
    <div className="flex flex-col items-center">
      <div className="m-4 w-full md:w-1/2">
        <h1 className="text-4xl">Public Feed</h1>
        <textarea
          value={newPost}
          onChange={(e) => setNewPost(e.target.value)}
          className="border p-2 m-2 w-full"
          placeholder="What's on your mind?"
        />
        <button onClick={handleNewPost} className="bg-blue-500 text-white p-2">Post</button>
        {feed.map((post: any, index: number) => (
          <div key={index} className="m-4 p-4 border">
            <h2 className="text-2xl">{post.username}</h2>
            <p>{post.content}</p>
            <p><small>Posted by {post.accountId}</small></p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ThePublicFeed;