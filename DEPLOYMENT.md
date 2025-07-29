# 🚀 Stark Squeeze Backend Deployment Guide

## 🎯 **Recommended: Railway Deployment**

Railway is the best free option for Rust backends with generous limits and easy deployment.

### 📋 **Prerequisites**
- GitHub account
- Railway account (free)
- Your Starknet configuration

### 🚀 **Step-by-Step Railway Deployment**

#### 1. **Prepare Your Repository**
```bash
# Ensure your code is committed to GitHub
git add .
git commit -m "Ready for Railway deployment"
git push origin main
```

#### 2. **Connect to Railway**
1. Go to [railway.app](https://railway.app)
2. Sign up with GitHub
3. Click "New Project"
4. Select "Deploy from GitHub repo"
5. Choose your `stark-squeeze` repository

#### 3. **Configure Environment Variables**
In Railway dashboard, go to your project → Variables tab and add:

```env
# Starknet Configuration
RPC_URL=https://starknet-sepolia.public.blastapi.io
CONTRACT_ADDRESS=0x64b10deedd5c3c40f3834e958877762eb2056029f077d6262f8e8f7c6396fe3
ACCOUNT_ADDRESS=your_account_address
PRIVATE_KEY=your_private_key
CHAIN_ID=0x534e5f4d41494e

# Pinata Configuration
PINATA_JWT=your_pinata_jwt_token

# Server Configuration
RUST_LOG=info
RUST_BACKTRACE=1
SERVER_PORT=8080
SERVER_HOST=0.0.0.0

# Feature Flags
ENABLE_STARKNET_UPLOAD=false
ENABLE_IPFS_UPLOAD=true
```

#### 4. **Deploy**
- Railway will automatically detect your `Dockerfile.backend`
- Build will start automatically
- Deployment takes 2-5 minutes

#### 5. **Get Your URL**
- Railway provides a URL like: `https://stark-squeeze-production.up.railway.app`
- Your API will be available at: `https://your-url.railway.app/health`

### 🧪 **Test Your Deployment**

```bash
# Health check
curl https://your-url.railway.app/health

# Test file upload
curl -X POST -F "file=@test_file.txt" https://your-url.railway.app/compress
```

## 🔧 **Alternative: Render Deployment**

### 1. **Create Render Account**
- Go to [render.com](https://render.com)
- Sign up with GitHub

### 2. **Deploy Web Service**
1. Click "New +" → "Web Service"
2. Connect your GitHub repo
3. Configure:
   - **Name:** `stark-squeeze-backend`
   - **Environment:** `Docker`
   - **Branch:** `main`
   - **Root Directory:** `/` (leave empty)

### 3. **Environment Variables**
Add the same environment variables as Railway above.

### 4. **Deploy**
- Render will build and deploy automatically
- URL format: `https://stark-squeeze-backend.onrender.com`

## 🔧 **Alternative: Fly.io Deployment**

### 1. **Install Fly CLI**
```bash
curl -L https://fly.io/install.sh | sh
```

### 2. **Login to Fly**
```bash
fly auth login
```

### 3. **Create App**
```bash
fly apps create stark-squeeze-backend
```

### 4. **Deploy**
```bash
fly deploy
```

## 📊 **Deployment Comparison**

| Platform | Free Tier | Pros | Cons |
|----------|-----------|------|------|
| **Railway** | $5/month credit | Easy setup, good docs | Limited free tier |
| **Render** | 750 hours/month | Simple, reliable | Sleeps after inactivity |
| **Fly.io** | 3 VMs, 3GB storage | Global edge, fast | More complex setup |

## 🔒 **Security Considerations**

### **Environment Variables**
- Never commit secrets to Git
- Use Railway/Render's secure environment variable storage
- Rotate keys regularly

### **SSL Certificates**
- Railway/Render provide automatic SSL
- Your API will be HTTPS by default

### **Rate Limiting**
- Consider adding rate limiting for production
- Monitor usage to stay within free limits

## 📈 **Monitoring & Scaling**

### **Railway Monitoring**
- Built-in logs and metrics
- Automatic restarts on failure
- Easy scaling with paid plans

### **Health Checks**
Your app includes health checks at `/health`:
```json
{
  "service": "stark-squeeze",
  "status": "healthy",
  "version": "1.0.0"
}
```

## 🚨 **Troubleshooting**

### **Common Issues**

1. **Build Fails**
   - Check Rust version compatibility
   - Verify Dockerfile syntax
   - Check build logs in Railway dashboard

2. **Environment Variables**
   - Ensure all required vars are set
   - Check variable names match code
   - Verify no extra spaces

3. **Health Check Fails**
   - Check if server starts properly
   - Verify port configuration
   - Check application logs

### **Debug Commands**
```bash
# Check Railway logs
railway logs

# Check Render logs
# Available in Render dashboard

# Check Fly logs
fly logs
```

## 🎯 **Next Steps**

1. **Deploy to Railway** (recommended)
2. **Test all endpoints**
3. **Update frontend to use new API URL**
4. **Monitor performance**
5. **Scale as needed**

---

**Need help?** Check Railway's excellent documentation or their Discord community! 