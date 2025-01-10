#!/bin/bash

# deploy.sh - Smooth and hassle-free deployments every time 🚀

set -e

# Configure these variables according to your setup
USER="your-username"
HOST="your-server-host"
REMOTE_DIR="/path/to/remote/directory"
LOCAL_DIR="/path/to/local/project"

# Ensure a commit message was provided
if [ -z "$1" ]; then
    echo "Usage: $0 <commit_message>"
    exit 1
fi

COMMIT_MESSAGE=$1

# Pull latest changes from git
echo "Adding, committing, and pushing changes to the repository..."
cd $LOCAL_DIR
git add .
git commit -m "$COMMIT_MESSAGE"
git push

# Send project files to the remote server
echo "Transferring files to the remote server..."
scp -r $LOCAL_DIR $USER@$HOST:$REMOTE_DIR

# SSH into the remote server and restart the application
echo "Deploying application on the remote server..."
ssh $USER@$HOST <<'ENDSSH'
set -e
cd /path/to/remote/directory
# Pull latest changes (if using git)
git pull
# Restart application (adjust to your specific setup)
docker-compose down
docker-compose up -d --build
ENDSSH

echo "Deployment completed successfully. Hooray! 🎉"