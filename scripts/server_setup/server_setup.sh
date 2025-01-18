#!/bin/bash

# Update system
sudo apt update
sudo apt upgrade -y

# Install required packages
sudo apt install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    software-properties-common \
    nginx \
    certbot \
    python3-certbot-nginx

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.23.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# Create deploy user
sudo useradd -m -s /bin/bash deploy
sudo usermod -aG docker deploy

# Setup deploy directory
sudo mkdir -p /opt/zibelina-store
sudo chown deploy:deploy /opt/zibelina-store

# Setup Nginx
cat > /etc/nginx/sites-available/zibelina << EOF
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_cache_bypass \$http_upgrade;
    }
}
EOF

sudo ln -s /etc/nginx/sites-available/zibelina /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

# Setup SSL
sudo certbot --nginx -d your-domain.com

# Create systemd service
cat > /etc/systemd/system/zibelina.service << EOF
[Unit]
Description=Zibelina Store API
After=network.target

[Service]
Type=simple
User=deploy
WorkingDirectory=/opt/zibelina-store
ExecStart=/usr/local/bin/docker-compose up
ExecStop=/usr/local/bin/docker-compose down
Restart=always

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl enable zibelina
sudo systemctl start zibelina

echo "Server setup completed!"