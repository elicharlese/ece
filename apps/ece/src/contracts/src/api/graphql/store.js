// Array of items. In a real application, this could come from a database.
const items = [
  { id: '1', name: 'Item 1', price: 29.99 },
  { id: '2', name: 'Item 2', price: 49.99 },
  { id: '3', name: 'Item 3', price: 19.99 },
];

// The resolver for the `items` query
const resolvers = {
  Query: {
    items: () => items,
  },
};

// Export the resolvers
module.exports = resolvers;