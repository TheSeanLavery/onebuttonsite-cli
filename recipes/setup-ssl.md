# Setup SSL/HTTPS

Configure SSL certificates for your custom domain. 

## How SSL Works with Amplify

When you add a domain to Amplify, SSL is **automatic**:
1. Amplify requests a certificate from AWS Certificate Manager
2. You add a verification DNS record
3. Certificate is issued and installed
4. HTTPS just works

## Prerequisites
- Domain added to Amplify (`obs domain` or `obs subdomain`)
- Access to DNS settings

## Steps

### 1. Check current SSL status
```bash
obs status <app-id>
```

Look for `sslValidation` in the output:
```json
{
  "domains": [{
    "domain": "example.com",
    "status": "PENDING_VERIFICATION",
    "sslValidation": "_abc123.example.com. CNAME _xyz789.acm-validations.aws."
  }]
}
```

### 2. Add the SSL validation record

If using Route53:
```bash
obs dns <zone-id> CNAME _abc123.example.com _xyz789.acm-validations.aws.
```

If using another registrar:
- Add a CNAME record in your DNS settings
- Name: `_abc123` (the part before your domain)
- Value: `_xyz789.acm-validations.aws.`

### 3. Wait for certificate issuance
```bash
obs status <app-id>
```

Status progression:
- `PENDING_VERIFICATION` → Add DNS record
- `REQUESTING_CERTIFICATE` → AWS is issuing cert
- `AVAILABLE` → SSL is active!

Takes 5-30 minutes.

### 4. Verify HTTPS works
```bash
curl -I https://yourdomain.com
```

Should return `HTTP/2 200`.

## What This Costs
- **SSL Certificate**: FREE (AWS Certificate Manager)
- **No renewal fees**: Auto-renews forever

## Troubleshooting

### Certificate stuck on PENDING_VERIFICATION

**Check DNS record exists:**
```bash
dig _abc123.example.com CNAME
```

Should return the `acm-validations.aws` value.

**Common issues:**
- Typo in the DNS record
- DNS not propagated yet (wait up to 48 hours)
- Wrong record type (must be CNAME, not TXT)

### Certificate failed

1. Delete the domain association:
```bash
aws amplify delete-domain-association \
  --app-id <app-id> \
  --domain-name example.com \
  --region us-east-1
```

2. Re-add it:
```bash
obs domain <app-id> example.com
```

3. Add the new DNS records

### Mixed content warnings

Your site loads over HTTPS but references HTTP resources.

Fix by:
- Using `//` protocol-relative URLs
- Or explicitly using `https://`
- Check for hardcoded `http://` in your HTML/CSS/JS

### HSTS / Force HTTPS

Amplify automatically redirects HTTP → HTTPS. No configuration needed.

## Manual Certificate (Advanced)

If you need a certificate for something other than Amplify:

```bash
# Request certificate
aws acm request-certificate \
  --domain-name example.com \
  --validation-method DNS \
  --subject-alternative-names "*.example.com" \
  --region us-east-1

# Get validation records
aws acm describe-certificate \
  --certificate-arn <arn> \
  --region us-east-1

# Add DNS records, then wait for validation
aws acm wait certificate-validated \
  --certificate-arn <arn> \
  --region us-east-1
```
