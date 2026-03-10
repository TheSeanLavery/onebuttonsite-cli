# Host a Frontend Site

Deploy and host a static frontend on AWS Amplify.

## What Amplify Hosting Gives You

- Global CDN (CloudFront edge locations)
- Automatic HTTPS/SSL
- Custom domains
- Instant cache invalidation on deploy
- No server management

## Supported Site Types

- Static HTML/CSS/JS
- React, Vue, Svelte, Angular SPAs
- Next.js (static export)
- Gatsby, Hugo, Jekyll builds
- Any static site generator output

## Steps

### 1. Prepare your site

Your site needs at minimum an `index.html` in the root.

**Plain HTML:**
```
my-site/
├── index.html
├── styles.css
└── script.js
```

**Built SPA (React, Vue, etc.):**
```bash
npm run build  # Creates dist/ or build/
```

### 2. Deploy to Amplify
```bash
obs create my-site ./dist
```

### 3. Get your URL
Output includes:
```json
{
  "url": "https://main.d1abc123.amplifyapp.com"
}
```

Site is live immediately.

### 4. (Optional) Add custom domain
```bash
# For root domain + www
obs domain <app-id> example.com

# For subdomain
obs subdomain <app-id> app example.com
```

## Updating Your Site

After making changes:
```bash
obs update <app-id> ./dist
```

Deploys in ~30 seconds, CDN cache invalidates automatically.

## Environment Variables

If your frontend needs env vars at build time:

```bash
aws amplify update-app \
  --app-id <app-id> \
  --environment-variables API_URL=https://api.example.com \
  --region us-east-1
```

Or add them in the Amplify Console.

## Single Page App (SPA) Routing

SPAs need all routes to serve `index.html`.

Add a redirect rule in Amplify Console, or create `_redirects` file:
```
/*    /index.html   200
```

Or via CLI:
```bash
aws amplify update-app \
  --app-id <app-id> \
  --custom-rules '[{"source":"/<*>","target":"/index.html","status":"200"}]' \
  --region us-east-1
```

## What This Costs

| Usage | Monthly Cost |
|-------|--------------|
| Small site (<5GB, <15GB transfer) | FREE |
| Medium site (10GB, 50GB transfer) | ~$3-5 |
| High traffic (10GB, 200GB transfer) | ~$15-25 |

See: `obs recipe show estimate-costs`

## Performance Tips

1. **Compress assets** - Gzip/Brotli your JS/CSS
2. **Optimize images** - Use WebP, lazy loading
3. **Cache headers** - Amplify sets good defaults
4. **Code split** - Reduce initial bundle size

## Full Example: React App

```bash
# Create React app
npx create-react-app my-app
cd my-app

# Build production bundle
npm run build

# Deploy to Amplify
obs create my-react-app ./build

# Update later
npm run build
obs update <app-id> ./build
```

## Full Example: Plain HTML

```bash
# Create directory
mkdir my-site && cd my-site

# Create index.html
cat > index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
  <title>My Site</title>
</head>
<body>
  <h1>Hello World</h1>
</body>
</html>
EOF

# Deploy
obs create my-site .

# Visit the URL from the output
```

## Troubleshooting

**Site shows old content**
- Cache might not have invalidated
- Force refresh: Ctrl+Shift+R
- Or redeploy: `obs update <app-id> ./dist`

**404 on page refresh (SPA)**
- Add SPA redirect rule (see above)

**Assets not loading**
- Check paths are relative (`./` or `/`) not absolute
- Check browser console for errors
