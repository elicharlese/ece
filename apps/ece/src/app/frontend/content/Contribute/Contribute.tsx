import React from 'react';
import useSWR from 'swr';

const fetcher = (url: string) => fetch(url).then(res => res.json());

const Contribute: React.FC = () => {
  const { data, error } = useSWR('/api/contribute', fetcher);

  if (error) return <div>Error loading data.</div>;
  if (!data) return <div>Loading...</div>;

  return (
    <div>
      <h1>Contribute Page</h1>
      <div>Data: {JSON.stringify(data)}</div>
    </div>
  );
};

export default Contribute;