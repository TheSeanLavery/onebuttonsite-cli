# Buy a Domain

Purchase a domain through AWS Route53 Domains.

## Prerequisites
- AWS CLI configured
- Payment method on AWS account
- Contact information ready

## ⚠️ This Costs Money
- `.com` domains: ~$12-15/year
- Charges appear on your AWS bill
- Auto-renews by default

## Steps

### 1. Check availability first
```bash
obs check-domain mycoolsite.com
```

Make sure `"available": true`.

### 2. Purchase via AWS Console (Recommended)

The safest way to buy:

1. Go to: https://console.aws.amazon.com/route53/home#DomainRegistration:
2. Search for your domain
3. Add to cart
4. Enter contact information
5. Complete purchase

### 3. Or via AWS CLI (Advanced)

```bash
aws route53domains register-domain \
  --domain-name mycoolsite.com \
  --duration-in-years 1 \
  --admin-contact '{
    "FirstName": "Your",
    "LastName": "Name",
    "ContactType": "PERSON",
    "Email": "you@email.com",
    "PhoneNumber": "+1.5551234567",
    "AddressLine1": "123 Main St",
    "City": "San Francisco",
    "State": "CA",
    "CountryCode": "US",
    "ZipCode": "94102"
  }' \
  --registrant-contact <same-as-above> \
  --tech-contact <same-as-above> \
  --privacy-protect-admin-contact \
  --privacy-protect-registrant-contact \
  --privacy-protect-tech-contact \
  --region us-east-1
```

### 4. Wait for registration
Domain registration takes 1-15 minutes. Check status:
```bash
aws route53domains get-domain-detail \
  --domain-name mycoolsite.com \
  --region us-east-1
```

### 5. Verify hosted zone was created
```bash
obs zones
```

You should see your new domain with a hosted zone ID.

## What You Get
- Domain registered for 1 year
- Auto-created Route53 Hosted Zone ($0.50/month)
- WHOIS privacy protection (free)
- Auto-renewal enabled

## Pricing

| TLD | Registration | Renewal |
|-----|--------------|---------|
| .com | $12 | $12 |
| .net | $11 | $11 |
| .org | $12 | $12 |
| .io | $39 | $39 |
| .co | $25 | $25 |

Plus: $0.50/month for the hosted zone.

## Next Steps

Once registered:
1. Add to your Amplify app: `obs domain <app-id> mycoolsite.com`
2. DNS records auto-configure since it's all AWS
3. SSL certificate auto-provisions

See: `obs recipe show setup-ssl`

## Troubleshooting

**Registration stuck "IN_PROGRESS"**
- Usually completes in 1-15 minutes
- Check email for verification (some TLDs require it)

**"Domain not available"**
- Someone else owns it
- Try a different TLD or name

**Contact verification email**
- Check spam folder
- Must verify within 15 days or domain is suspended
