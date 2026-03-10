# Host a VPS

Set up a Virtual Private Server on AWS EC2 for backends, APIs, or full-stack apps.

## When You Need a VPS

- Running a backend server (Node, Python, Go, etc.)
- Databases that need persistence
- WebSocket servers
- Cron jobs / background workers
- Anything that isn't static files

## Quick Start

### 1. Launch an EC2 instance

**Via Console (Easier for first time):**
https://console.aws.amazon.com/ec2/v2/home#LaunchInstances

Choose:
- **AMI**: Ubuntu 24.04 LTS
- **Instance type**: t3.micro (free tier) or t3.small
- **Key pair**: Create new or use existing
- **Security group**: Allow SSH (22), HTTP (80), HTTPS (443)

**Via CLI:**
```bash
# Create key pair
aws ec2 create-key-pair \
  --key-name my-key \
  --query 'KeyMaterial' \
  --output text > my-key.pem
chmod 400 my-key.pem

# Get latest Ubuntu AMI
AMI=$(aws ec2 describe-images \
  --owners 099720109477 \
  --filters "Name=name,Values=ubuntu/images/hvm-ssd/ubuntu-*-24.04-amd64-server-*" \
  --query 'Images | sort_by(@, &CreationDate) | [-1].ImageId' \
  --output text)

# Launch instance
aws ec2 run-instances \
  --image-id $AMI \
  --instance-type t3.micro \
  --key-name my-key \
  --security-group-ids <sg-id> \
  --tag-specifications 'ResourceType=instance,Tags=[{Key=Name,Value=my-server}]'
```

### 2. Connect via SSH
```bash
# Get public IP
aws ec2 describe-instances \
  --filters "Name=tag:Name,Values=my-server" \
  --query 'Reservations[0].Instances[0].PublicIpAddress' \
  --output text

# Connect
ssh -i my-key.pem ubuntu@<public-ip>
```

### 3. Set up your server
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Node.js (example)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Install PM2 for process management
sudo npm install -g pm2

# Clone your app
git clone https://github.com/you/your-app.git
cd your-app
npm install

# Start with PM2
pm2 start server.js
pm2 startup
pm2 save
```

### 4. Point domain to VPS
```bash
# Get the IP
IP=$(aws ec2 describe-instances \
  --filters "Name=tag:Name,Values=my-server" \
  --query 'Reservations[0].Instances[0].PublicIpAddress' \
  --output text)

# Add A record
obs dns <zone-id> A api.example.com $IP
```

### 5. Set up SSL with Certbot
```bash
# On the server
sudo apt install certbot python3-certbot-nginx -y
sudo certbot --nginx -d api.example.com
```

## Instance Types & Costs

| Type | vCPU | RAM | Cost/month |
|------|------|-----|------------|
| t3.micro | 2 | 1GB | ~$8 (free tier eligible) |
| t3.small | 2 | 2GB | ~$15 |
| t3.medium | 2 | 4GB | ~$30 |
| t3.large | 2 | 8GB | ~$60 |

Plus:
- EBS storage: ~$0.10/GB/month
- Data transfer out: $0.09/GB (first 100GB/mo free)

## Security Best Practices

### Security group rules
```bash
# Create security group
aws ec2 create-security-group \
  --group-name web-server \
  --description "Web server access"

# Allow SSH only from your IP
aws ec2 authorize-security-group-ingress \
  --group-name web-server \
  --protocol tcp \
  --port 22 \
  --cidr $(curl -s ifconfig.me)/32

# Allow HTTP/HTTPS from anywhere
aws ec2 authorize-security-group-ingress \
  --group-name web-server \
  --protocol tcp \
  --port 80 \
  --cidr 0.0.0.0/0

aws ec2 authorize-security-group-ingress \
  --group-name web-server \
  --protocol tcp \
  --port 443 \
  --cidr 0.0.0.0/0
```

### On the server
```bash
# Disable password auth (use keys only)
sudo sed -i 's/PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
sudo systemctl restart sshd

# Set up firewall
sudo ufw allow OpenSSH
sudo ufw allow 'Nginx Full'
sudo ufw enable

# Auto security updates
sudo apt install unattended-upgrades -y
sudo dpkg-reconfigure -plow unattended-upgrades
```

## Common Setups

### Node.js API
```bash
sudo apt install -y nodejs npm
npm install -g pm2
pm2 start app.js --name api
pm2 startup && pm2 save
```

### Python/FastAPI
```bash
sudo apt install -y python3 python3-pip python3-venv
python3 -m venv venv
source venv/bin/activate
pip install fastapi uvicorn
uvicorn main:app --host 0.0.0.0 --port 8000
```

### Docker
```bash
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker ubuntu
# Log out and back in
docker run -d -p 80:80 nginx
```

### Nginx reverse proxy
```bash
sudo apt install -y nginx
sudo tee /etc/nginx/sites-available/api << 'EOF'
server {
    listen 80;
    server_name api.example.com;
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
EOF
sudo ln -s /etc/nginx/sites-available/api /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

## Managing the Instance

```bash
# Stop (still charged for EBS)
aws ec2 stop-instances --instance-ids <id>

# Start
aws ec2 start-instances --instance-ids <id>

# Terminate (deletes everything)
aws ec2 terminate-instances --instance-ids <id>
```

## Troubleshooting

**Can't connect via SSH**
- Check security group allows port 22
- Check instance is running
- Check you're using the correct key and username

**Site not accessible**
- Check security group allows port 80/443
- Check your app is running: `pm2 status`
- Check nginx config: `sudo nginx -t`
