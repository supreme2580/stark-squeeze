# 🐳 Docker Backend Deployment

This document provides instructions for deploying the Stark Squeeze backend using Docker and Docker Compose.

## 📋 Prerequisites

- Docker Engine 20.10+
- Docker Compose 2.0+
- At least 4GB RAM available
- 10GB free disk space

## 🚀 Quick Start

### 1. Build the Backend Image

```bash
# Build the backend Docker image
./scripts/docker-build.sh
```

### 2. Deploy All Services

```bash
# Deploy all services (development)
docker-compose up -d

# Or use the deployment script
./scripts/docker-deploy.sh
```

### 3. Check Service Status

```bash
# View running services
docker-compose ps

# View logs
docker-compose logs -f
```

## 🏗️ Architecture

The Docker setup includes the following services:

- **Backend**: Rust application (port 8080)
- **PostgreSQL**: Database for file metadata (port 5432)
- **Redis**: Cache for session management (port 6379)
- **IPFS**: File storage node (ports 4001, 5001, 8080)
- **Nginx**: Reverse proxy (ports 80, 443)

## 📁 File Structure

```
stark-squeeze/
├── Dockerfile.backend          # Backend Dockerfile
├── docker-compose.yml          # Development compose
├── docker-compose.prod.yml     # Production compose
├── .dockerignore              # Docker ignore file
├── scripts/
│   ├── docker-build.sh        # Build script
│   └── docker-deploy.sh       # Deploy script
└── DOCKER_README.md           # This file
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
# Database
POSTGRES_DB=starksqueeze
POSTGRES_USER=starksqueeze
POSTGRES_PASSWORD=your_secure_password
DATABASE_URL=postgresql://starksqueeze:your_secure_password@postgres:5432/starksqueeze

# Redis
REDIS_PASSWORD=your_redis_password
REDIS_URL=redis://:your_redis_password@redis:6379

# IPFS
IPFS_API_URL=http://ipfs:5001/api/v0

# Starknet
STARKNET_RPC_URL=https://alpha-mainnet.starknet.io

# Security
JWT_SECRET=your_jwt_secret_key
API_KEY=your_api_key

# Logging
RUST_LOG=info
RUST_BACKTRACE=0
```

### Production Deployment

For production deployment, use the production compose file:

```bash
# Deploy with production settings
docker-compose -f docker-compose.prod.yml up -d
```

## 🛠️ Commands

### Development

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down

# Restart services
docker-compose restart

# Rebuild and start
docker-compose up -d --build
```

### Production

```bash
# Deploy production stack
docker-compose -f docker-compose.prod.yml up -d

# View production logs
docker-compose -f docker-compose.prod.yml logs -f

# Stop production stack
docker-compose -f docker-compose.prod.yml down
```

### Individual Services

```bash
# Start only backend
docker-compose up -d backend

# Start only database
docker-compose up -d postgres

# View specific service logs
docker-compose logs -f backend
```

## 🔍 Health Checks

All services include health checks:

- **Backend**: `http://localhost:8080/health`
- **PostgreSQL**: Database connectivity check
- **Redis**: Cache connectivity check
- **IPFS**: Node status check

## 📊 Monitoring

### View Service Status

```bash
# Check all services
docker-compose ps

# Check specific service
docker-compose ps backend
```

### View Resource Usage

```bash
# View container stats
docker stats

# View specific container stats
docker stats starksqueeze-backend-1
```

### View Logs

```bash
# All services
docker-compose logs

# Specific service
docker-compose logs backend

# Follow logs
docker-compose logs -f backend

# Last 100 lines
docker-compose logs --tail=100 backend
```

## 🔒 Security

### Production Security Features

- Non-root user in containers
- Resource limits and reservations
- Network isolation
- Secrets management via environment variables
- Health checks for all services

### Security Best Practices

1. **Change default passwords** in `.env` file
2. **Use strong secrets** for JWT and API keys
3. **Limit port exposure** in production
4. **Regular security updates** of base images
5. **Monitor logs** for suspicious activity

## 🚨 Troubleshooting

### Common Issues

#### 1. Port Conflicts

```bash
# Check if ports are in use
netstat -tulpn | grep :8080

# Change ports in docker-compose.yml
ports:
  - "8081:8080"  # Use different host port
```

#### 2. Build Failures

```bash
# Clean build cache
docker system prune -a

# Rebuild without cache
docker-compose build --no-cache
```

#### 3. Database Connection Issues

```bash
# Check database logs
docker-compose logs postgres

# Restart database
docker-compose restart postgres
```

#### 4. Memory Issues

```bash
# Check memory usage
docker stats

# Increase Docker memory limit in Docker Desktop
```

### Debug Commands

```bash
# Enter container shell
docker-compose exec backend sh

# Check container logs
docker-compose logs backend

# Check container status
docker-compose ps

# View container details
docker inspect starksqueeze-backend-1
```

## 📈 Performance

### Resource Requirements

- **Backend**: 512MB RAM, 0.5 CPU
- **PostgreSQL**: 256MB RAM, 0.25 CPU
- **Redis**: 128MB RAM, 0.1 CPU
- **IPFS**: 512MB RAM, 0.25 CPU
- **Nginx**: 128MB RAM, 0.1 CPU

### Optimization Tips

1. **Use production compose** for better performance
2. **Adjust resource limits** based on usage
3. **Monitor resource usage** with `docker stats`
4. **Use volume mounts** for persistent data
5. **Enable logging rotation** for large deployments

## 🔄 Updates

### Updating the Application

```bash
# Pull latest changes
git pull

# Rebuild and restart
docker-compose down
docker-compose up -d --build
```

### Updating Base Images

```bash
# Update all images
docker-compose pull

# Restart with new images
docker-compose up -d
```

## 📝 Logs

### Log Locations

- **Application logs**: `/app/logs/` in backend container
- **Nginx logs**: `/var/log/nginx/` in nginx container
- **Database logs**: PostgreSQL internal logging
- **Docker logs**: `docker-compose logs`

### Log Rotation

Configure log rotation in production:

```yaml
# In docker-compose.prod.yml
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
```

## 🆘 Support

For issues with the Docker deployment:

1. Check the troubleshooting section above
2. Review container logs: `docker-compose logs`
3. Verify environment variables are set correctly
4. Ensure Docker has sufficient resources
5. Check network connectivity between services

## 📚 Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [PostgreSQL Docker Image](https://hub.docker.com/_/postgres)
- [Redis Docker Image](https://hub.docker.com/_/redis)
- [IPFS Docker Image](https://hub.docker.com/r/ipfs/kubo) 