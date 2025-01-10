#!/bin/bash

# Build the applications
nx build ece
nx build portable

# Move to the output directory of the built applications
cd dist/apps/ece

# Deploy ece app using Docker
docker build -t ece-app .
docker run -d -p 3000:3000 ece-app