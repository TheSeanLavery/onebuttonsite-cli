# Add a Subdomain

Connect a subdomain (like blog.example.com) to your Amplify app.

## Prerequisites
- An existing Amplify app (`obs apps` to list)
- A domain you own (in Route53 or elsewhere)
- The hosted zone ID (`obs zones` to list)

## Steps

### 1. Find your app ID
```bash
obs apps
```

Note the `appId` of the app you want to add the subdomain to.

### 2. Add the subdomain to Amplify
```bash
obs subdomain <app-id> <prefix> <domain>
```

Example:
```bash
obs subdomain d1abc123xyz blog example.com
```

This creates: `blog.example.com`

### 3. Get the DNS record
The output shows what DNS record to add:
```json
{
  "ok": true,
  "subdomain": "blog.example.com",
  "status": "CREATING",
  "dnsRecordNeeded": "blog CNAME d123xyz.cloudfront.net"
}
```

### 4. Add DNS record to Route53
```bash
obs dns <zone-id> CNAME blog.example.com d123xyz.cloudfront.net
```

Get zone-id from `obs zones`.

### 5. Wait for SSL
Check status:
```bash
obs status <app-id>
```

Status progression:
1. `CREATING` - Setting up
2. `PENDING_VERIFICATION` - Waiting for DNS
3. `REQUESTING_CERTIFICATE` - Getting SSL cert
4. `AVAILABLE` - Ready to use!

Takes 5-15 minutes.

## Full Example

```bash
# List apps to find app ID
obs apps
# → appId: d1abc123xyz

# List zones to find zone ID  
obs zones
# → id: Z1234567890ABC, name: example.com

# Add subdomain
obs subdomain d1abc123xyz blog example.com
# → dnsRecordNeeded: "blog CNAME d9xyz.cloudfront.net"

# Add DNS record
obs dns Z1234567890ABC CNAME blog.example.com d9xyz.cloudfront.net

# Check status (repeat until AVAILABLE)
obs status d1abc123xyz
```

## What This Costs
- **Subdomain setup**: FREE
- **DNS queries**: ~$0.40 per million queries (negligible)
- **SSL certificate**: FREE (AWS Certificate Manager)

## Using a Domain from Another Registrar

If your domain is NOT in Route53:

1. Add the subdomain to Amplify (same as above)
2. Go to your registrar's DNS settings
3. Add a CNAME record:
   - Name: `blog` (or your prefix)
   - Type: `CNAME`
   - Value: `d123xyz.cloudfront.net` (from step 3)
4. Wait for DNS propagation (can take up to 48 hours)

## Troubleshooting

**Stuck on PENDING_VERIFICATION**
- DNS record not added or not propagated yet
- Verify: `dig blog.example.com CNAME`
- Wait up to 48 hours for propagation

**SSL certificate failed**
- Usually means DNS isn't pointing to CloudFront
- Check `obs status <app-id>` for the exact CNAME needed

**"Domain association already exists"**
- This domain is already linked to an app
- Remove it first or use a different subdomain
