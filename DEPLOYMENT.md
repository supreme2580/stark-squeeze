# 🚀 Deployment Guide: Stark Squeeze Server

## Overview
This guide provides step-by-step instructions to deploy the Stark Squeeze Rust project on any server. The deployment is designed to be server-agnostic and includes production-ready configurations.

## 📋 Prerequisites

### System Requirements
- Linux server (Ubuntu 18.04+, CentOS 7+, or similar)
- 2GB+ RAM
- 10GB+ disk space
- Root/sudo access

### Required Software
- Git
- Rust toolchain
- Build essentials
- Systemd (for service management)

## 🔧 Step-by-Step Deployment

### 1. **Install Prerequisites**

```bash
# Update package manager
sudo apt update  # Ubuntu/Debian
# OR
sudo yum update  # CentOS/RHEL

# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install build essentials
sudo apt install build-essential pkg-config libssl-dev  # Ubuntu/Debian
# OR
sudo yum groupinstall "Development Tools"  # CentOS/RHEL

# Install additional dependencies
sudo apt install git curl wget  # Ubuntu/Debian
# OR
sudo yum install git curl wget  # CentOS/RHEL
```

### 2. **Clone and Setup Project**

```bash
# Clone the repository
git clone <your-repository-url>
cd stark-squeeze

# Verify Rust installation
rustc --version
cargo --version
```

### 3. **Build the Project**

```bash
# Build in release mode for optimal performance
cargo build --release

# Verify binaries were created
ls -la target/release/
# Should show: stark_squeeze and server binaries
```

### 4. **Environment Setup**

```bash
# Create necessary directories
mkdir -p /opt/stark-squeeze
mkdir -p /var/log/stark-squeeze
mkdir -p /etc/stark-squeeze

# Copy binaries to system location
sudo cp target/release/server /opt/stark-squeeze/
sudo cp target/release/stark_squeeze /opt/stark-squeeze/

# Make binaries executable
sudo chmod +x /opt/stark-squeeze/server
sudo chmod +x /opt/stark-squeeze/stark_squeeze

# Copy configuration files
sudo cp config.json /etc/stark-squeeze/
sudo cp ascii_combinations.json /etc/stark-squeeze/ 2>/dev/null || echo "Dictionary will be generated on first run"
```

### 5. **Create Systemd Service**

```bash
# Create systemd service file
sudo tee /etc/systemd/system/stark-squeeze.service > /dev/null <<EOF
[Unit]
Description=Stark Squeeze Server
After=network.target

[Service]
Type=simple
User=stark-squeeze
Group=stark-squeeze
WorkingDirectory=/opt/stark-squeeze
ExecStart=/opt/stark-squeeze/server
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
Environment=RUST_LOG=info
Environment=ENABLE_STARKNET_UPLOAD=false

[Install]
WantedBy=multi-user.target
EOF

# Create user for the service
sudo useradd -r -s /bin/false stark-squeeze

# Set ownership
sudo chown -R stark-squeeze:stark-squeeze /opt/stark-squeeze
sudo chown -R stark-squeeze:stark-squeeze /etc/stark-squeeze
sudo chown -R stark-squeeze:stark-squeeze /var/log/stark-squeeze
```

### 6. **Configure Firewall**

```bash
# Allow HTTP traffic on port 3000
sudo ufw allow 3000/tcp  # Ubuntu/Debian
# OR
sudo firewall-cmd --permanent --add-port=3000/tcp  # CentOS/RHEL
sudo firewall-cmd --reload  # CentOS/RHEL
```

### 7. **Start the Service**

```bash
# Reload systemd and enable service
sudo systemctl daemon-reload
sudo systemctl enable stark-squeeze
sudo systemctl start stark-squeeze

# Check service status
sudo systemctl status stark-squeeze

# View logs
sudo journalctl -u stark-squeeze -f
```

### 8. **Verify Deployment**

```bash
# Test health endpoint
curl http://localhost:3000/health

# Test status endpoint
curl http://localhost:3000/status

# Test file compression (replace with actual file)
curl -X POST http://localhost:3000/compress \
  -F "file=@/path/to/test/file.txt"
```

## 🌐 Optional: Reverse Proxy Setup

### Nginx Configuration

```bash
# Install Nginx
sudo apt install nginx  # Ubuntu/Debian
# OR
sudo yum install nginx  # CentOS/RHEL

# Create Nginx configuration
sudo tee /etc/nginx/sites-available/stark-squeeze > /dev/null <<EOF
server {
    listen 80;
    server_name your-domain.com;  # Replace with your domain

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF

# Enable site
sudo ln -s /etc/nginx/sites-available/stark-squeeze /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### SSL/HTTPS Setup

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx  # Ubuntu/Debian
# OR
sudo yum install certbot python3-certbot-nginx  # CentOS/RHEL

# Get SSL certificate
sudo certbot --nginx -d your-domain.com
```

## 🔧 Environment Configuration

### Environment Variables

```bash
# Create environment file
sudo tee /etc/stark-squeeze/.env > /dev/null <<EOF
ENABLE_STARKNET_UPLOAD=false
RUST_LOG=info
RUST_BACKTRACE=1
EOF

# Update service to use environment file
sudo sed -i 's|Environment=RUST_LOG=info|EnvironmentFile=/etc/stark-squeeze/.env|' /etc/systemd/system/stark-squeeze.service
sudo systemctl daemon-reload
sudo systemctl restart stark-squeeze
```

## 📊 Monitoring and Maintenance

### Service Management

```bash
# Check service status
sudo systemctl status stark-squeeze

# View real-time logs
sudo journalctl -u stark-squeeze -f

# Restart service
sudo systemctl restart stark-squeeze

# Stop service
sudo systemctl stop stark-squeeze

# Disable service
sudo systemctl disable stark-squeeze
```

### Application Updates

```bash
# Pull latest changes
git pull

# Rebuild application
cargo build --release

# Update binaries
sudo cp target/release/server /opt/stark-squeeze/
sudo cp target/release/stark_squeeze /opt/stark-squeeze/

# Restart service
sudo systemctl restart stark-squeeze
```

### Log Management

```bash
# View recent logs
sudo journalctl -u stark-squeeze -n 100

# View logs since last boot
sudo journalctl -u stark-squeeze -b

# Clear logs (if needed)
sudo journalctl --vacuum-time=7d
```

## 🔍 Troubleshooting

### Common Issues

1. **Service won't start**
   ```bash
   # Check logs for errors
   sudo journalctl -u stark-squeeze -n 50
   
   # Verify binary exists and is executable
   ls -la /opt/stark-squeeze/server
   ```

2. **Port already in use**
   ```bash
   # Check what's using port 3000
   sudo netstat -tlnp | grep :3000
   
   # Kill process if needed
   sudo kill -9 <PID>
   ```

3. **Permission issues**
   ```bash
   # Fix ownership
   sudo chown -R stark-squeeze:stark-squeeze /opt/stark-squeeze
   sudo chown -R stark-squeeze:stark-squeeze /etc/stark-squeeze
   ```

4. **Dictionary generation fails**
   ```bash
   # Check disk space
   df -h
   
   # Manually generate dictionary
   cargo run --bin stark_squeeze
   ```

### Health Checks

```bash
# Test all endpoints
curl -s http://localhost:3000/health | jq .
curl -s http://localhost:3000/status | jq .

# Test file upload
echo "test content" > test.txt
curl -X POST http://localhost:3000/compress \
  -F "file=@test.txt" | jq .
```

## 📝 Configuration Notes

- **Port**: Default is 3000, change in `src/server.rs` if needed
- **Dictionary**: Auto-generated on first run if `ascii_combinations.json` doesn't exist
- **Starknet**: Disabled by default, enable via `ENABLE_STARKNET_UPLOAD=true`
- **Logs**: Available via `journalctl -u stark-squeeze`
- **Updates**: Simply rebuild and restart the service

## 🚨 Security Considerations

1. **Firewall**: Ensure only necessary ports are open
2. **User**: Service runs as non-root user `stark-squeeze`
3. **SSL**: Use HTTPS in production
4. **Updates**: Keep system and dependencies updated
5. **Monitoring**: Set up log monitoring and alerting

## 📞 Support

If you encounter issues:
1. Check the logs: `sudo journalctl -u stark-squeeze -f`
2. Verify service status: `sudo systemctl status stark-squeeze`
3. Test endpoints: `curl http://localhost:3000/health`
4. Create an issue with logs and error details

---

**Note**: This deployment guide is designed to be server-agnostic. Adjust package manager commands (`apt` vs `yum`) based on your Linux distribution. 