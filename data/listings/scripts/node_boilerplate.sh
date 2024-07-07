#!/bin/bash

# node_boilerplate.sh - Your new best friend for creating Node.js projects

set -e

# Check if a project name was provided
if [ -z "$1" ]; then
    echo "Usage: $0 <project_name>"
    exit 1
fi

PROJECT_NAME=$1

# Create project directory
if [ -d "$PROJECT_NAME" ]; then
    echo "Directory $PROJECT_NAME already exists. Please choose a different name."
    exit 1
fi

echo "Creating project directory..."
mkdir $PROJECT_NAME
cd $PROJECT_NAME

# Initialize npm project
echo "Initializing npm project..."
npm init -y

# Create basic project structure
echo "Creating project structure..."
mkdir -p src tests
touch src/index.js
touch tests/test.js

# Install common dependencies
echo "Installing dependencies..."
npm install express
npm install mocha chai supertest --save-dev

# Create basic Express server code
cat <<EOF > src/index.js
const express = require('express');
const app = express();
const PORT = process.env.PORT || 3000;

app.get('/', (req, res) => {
  res.send('Hello, World!');
});

app.listen(PORT, () => {
  console.log(\`Server is running on port \${PORT}\`);
});

module.exports = app;
EOF

# Create basic test file
cat <<EOF > tests/test.js
const request = require('supertest');
const app = require('../src/index');

describe("Basic Test", () => {
  it("should return Hello, World on GET /", (done) => {
    request(app)
      .get('/')
      .expect('Hello, World!', done);
  });
});
EOF

echo "Node.js project boilerplate created successfully. Now go build something awesome! 🚀"