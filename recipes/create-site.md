# Create and Deploy a New Site

Deploy a static site to AWS Amplify in one command.

## Prerequisites
- AWS CLI configured (`aws configure`)
- A directory with your site files (at minimum: `index.html`)

## Steps

### 1. Verify AWS credentials
```bash
obs doctor
```
Expected: `"ok": true` with your account info.

### 2. Create and deploy the site
```bash
obs create <site-name> <directory>
```

Example:
```bash
obs create my-portfolio ./dist
```

### 3. Note the output
The command returns:
- `appId` - You'll need this for updates and domain setup
- `url` - Your live site URL (https://main.{appId}.amplifyapp.com)

### 4. Verify it's live
Visit the URL in your browser.

## Full Example

```bash
# Check credentials
obs doctor

# Create site directory
mkdir -p ./my-site
echo "<h1>Hello World</h1>" > ./my-site/index.html

# Deploy
obs create hello-world ./my-site

# Output:
# {
#   "ok": true,
#   "appId": "d1abc123xyz",
#   "url": "https://main.d1abc123xyz.amplifyapp.com"
# }
```

## What This Costs
- **Amplify Hosting**: Free tier covers 5GB storage, 15GB/month transfer
- **Beyond free tier**: ~$0.023/GB stored, ~$0.15/GB transferred
- **Typical small site**: $0-5/month

## Next Steps
- Add a custom domain: `obs recipe show add-subdomain`
- Update the site later: `obs update <appId> ./my-site`
- Check status: `obs status <appId>`

## Troubleshooting

**"Directory not found"**
- Make sure the path exists and contains files

**"AWS credentials not configured"**
- Run `aws configure` and enter your access keys

**Deployment stuck**
- Check AWS Console: https://console.aws.amazon.com/amplify/
- Try `obs status <appId>` to see deployment state
