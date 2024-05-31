# Use an official Node runtime as a parent image
FROM node:14 AS builder

# Set the working directory in the container for Node.js build
WORKDIR /app

# Copy package.json and package-lock.json for Node.js dependencies
COPY package*.json ./

# Install Node.js dependencies
RUN npm install

# Copy the rest of the Node.js application
COPY . .

# Build the Node.js application
RUN npm run build

# Use jlesage/baseimage-gui as the base image for the GUI application
FROM jlesage/baseimage-gui:debian-11-v4.4.0

ENV VERSION 0.65.6
ENV ARCHIVE https://github.com/buchen/portfolio/releases/download/${VERSION}/PortfolioPerformance-${VERSION}-linux.gtk.x86_64.tar.gz
ENV APP_ICON_URL=https://www.portfolio-performance.info/images/logo.png

# Install dependencies and configure PortfolioPerformance
RUN apt-get update && apt-get install -y wget openjdk-17-jre libwebkit2gtk-4.0-37 && \
    cd /opt && wget ${ARCHIVE} && tar xvzf PortfolioPerformance-${VERSION}-linux.gtk.x86_64.tar.gz && \
    rm PortfolioPerformance-${VERSION}-linux.gtk.x86_64.tar.gz && \
    echo "-data\n/config/portfolio\n$(cat /opt/portfolio/PortfolioPerformance.ini)" > /opt/portfolio/PortfolioPerformance.ini && \
    echo "osgi.nl=en" >> /opt/portfolio/configuration/config.ini && \
    chmod -R 777 /opt/portfolio && \
    install_app_icon.sh "$APP_ICON_URL"

# Copy Node.js application build output from builder stage
COPY --from=builder /app /app

# Set the working directory to the Node.js application
WORKDIR /app

# Expose the application port
EXPOSE 3000

# Start the Node.js application
CMD ["npm", "start"]