#!/bin/bash

# env_setup.sh - A friendly guide to setting up your development environment

set -e

# Function to check if a command exists
command_exists() {
    command -v "$1" &> /dev/null
}

echo "Welcome! Let's set up your development environment. 🚀"

# Update package lists
echo "Updating package lists..."
sudo apt-get update -y

# Install Git
if command_exists git; then
    echo "Git is already installed. Skipping."
else
    echo "Installing Git..."
    sudo apt-get install git -y
fi

# Install Node.js and npm
if command_exists node && command_exists npm; then
    echo "Node.js and npm are already installed. Skipping."
else
    echo "Installing Node.js and npm..."
    sudo apt-get install nodejs npm -y
fi

# Install Python and pip
if command_exists python3 && command_exists pip3; then
    echo "Python and pip are already installed. Skipping."
else
    echo "Installing Python and pip..."
    sudo apt-get install python3 python3-pip -y
fi

# Install Docker
if command_exists docker; then
    echo "Docker is already installed. Skipping."
else
    echo "Installing Docker..."
    sudo apt-get install docker.io -y
    sudo systemctl start docker
    sudo systemctl enable docker
fi

# Install VS Code
if command_exists code; then
    echo "Visual Studio Code is already installed. Skipping."
else
    echo "Installing Visual Studio Code..."
    wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > microsoft.gpg
    sudo install -o root -g root -m 644 microsoft.gpg /etc/apt/trusted.gpg.d/
    sudo sh -c 'echo "deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main" > /etc/apt/sources.list.d/vscode.list'
    sudo apt-get update -y
    sudo apt-get install code -y
    rm -f microsoft.gpg
fi

echo "Your development environment is ready! Happy coding! 🎉"