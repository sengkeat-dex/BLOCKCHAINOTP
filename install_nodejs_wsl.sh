#!/bin/bash

# Script to install Node.js 20 in WSL

echo "Installing Node.js 20 in WSL..."

# Try to install without sudo first (some WSL installations allow this)
echo "Attempting installation without sudo..."

# Update package lists
apt update

# Install curl if not already installed
apt install -y curl

# Add NodeSource repository for Node.js 20
curl -fsSL https://deb.nodesource.com/setup_20.x | bash -

# Install Node.js
apt-get install -y nodejs

# If the above fails, try with sudo
if [ $? -ne 0 ]; then
    echo "Attempting installation with sudo..."
    
    # Update package lists
    sudo apt update

    # Install curl if not already installed
    sudo apt install -y curl

    # Add NodeSource repository for Node.js 20
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -

    # Install Node.js
    sudo apt-get install -y nodejs
fi

# Verify installation
echo "Node.js version:"
node --version

echo "npm version:"
npm --version

echo "Installation complete!"