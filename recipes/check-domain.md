# Check Domain Availability

Check if a domain is available to purchase through AWS Route53.

## Prerequisites
- AWS CLI configured with Route53 access

## Steps

### 1. Check a single domain
```bash
obs check-domain <domain-name>
```

Example:
```bash
obs check-domain mycoolsite.com
```

### 2. Read the result
```json
{
  "ok": true,
  "domain": "mycoolsite.com",
  "available": true,
  "status": "AVAILABLE",
  "price": "~$12-15/year for .com"
}
```

Or if taken:
```json
{
  "ok": true,
  "domain": "google.com",
  "available": false,
  "status": "UNAVAILABLE",
  "price": null
}
```

## Supported TLDs

AWS Route53 supports most common TLDs:
- `.com`, `.net`, `.org` (~$12-15/year)
- `.io` (~$39/year)
- `.co` (~$25/year)
- `.app` (~$16/year)

**NOT supported by AWS:**
- `.dev` (use Google Domains)
- Some country-specific TLDs

## Bulk Checking (Manual)

To check multiple domains:
```bash
for domain in site1.com site2.com site3.com; do
  obs check-domain $domain
done
```

## What This Costs
- **Checking availability**: FREE
- **No charges** until you actually purchase

## Next Steps

If available and you want to buy:
- See `obs recipe show buy-domain`

If you already own the domain elsewhere:
- You can still use it with Amplify
- See `obs recipe show add-subdomain`

## Troubleshooting

**"UnsupportedTLD" error**
- AWS doesn't support this TLD
- Try a different extension (.com, .net, etc.)
- Or purchase from another registrar (Namecheap, Google Domains)

**"Access Denied"**
- Your AWS user needs `route53domains:CheckDomainAvailability` permission
