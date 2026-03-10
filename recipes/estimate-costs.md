# Estimate Costs

Understand what things cost before you do them.

## Free Tier (12 months)

New AWS accounts get free tier for 12 months:

| Service | Free Allowance |
|---------|----------------|
| Amplify Hosting | 5GB storage, 15GB transfer/month |
| Route53 Hosted Zone | 1 zone free |
| EC2 (t2.micro) | 750 hours/month |
| S3 | 5GB storage |
| Lambda | 1M requests/month |
| CloudFront | 1TB transfer/month |

## Cost by Action

### Creating a Site (`obs create`)
| Component | Cost |
|-----------|------|
| Amplify App | FREE |
| Build minutes | 1000 free/month, then $0.01/min |
| Hosting (5GB) | FREE tier, then $0.023/GB |
| Transfer (15GB) | FREE tier, then $0.15/GB |

**Typical small site: $0-2/month**

### Checking Domain (`obs check-domain`)
| Component | Cost |
|-----------|------|
| Availability check | FREE |

### Buying a Domain (`buy-domain`)
| Component | Cost |
|-----------|------|
| .com registration | $12/year |
| .io registration | $39/year |
| Hosted zone | $0.50/month |
| DNS queries | $0.40 per million |

**Typical: $12-15/year + $6/year for hosted zone**

### Adding Subdomain (`obs subdomain`)
| Component | Cost |
|-----------|------|
| Domain association | FREE |
| SSL certificate | FREE |
| DNS record | Included in hosted zone |

**Cost: $0**

### Hosting a VPS (`hosting-vps`)
| Instance | Cost |
|----------|------|
| t2.micro | FREE tier (750 hrs/mo) or ~$8/month |
| t3.small | ~$15/month |
| t3.medium | ~$30/month |

Plus:
- EBS storage: $0.10/GB/month
- Elastic IP (if stopped): $0.005/hour
- Data transfer: $0.09/GB out

## Monthly Cost Calculator

### Small Static Site (Blog/Portfolio)
```
Amplify Hosting:     $0 (within free tier)
Domain (.com):       $1/month ($12/year)
Hosted Zone:         $0.50/month
DNS queries:         $0 (negligible)
SSL:                 $0
─────────────────────────────
Total:               ~$1.50/month
```

### Medium Traffic Site (10K visitors/month)
```
Amplify Hosting:     $0-2 (maybe exceed free tier)
Domain (.com):       $1/month
Hosted Zone:         $0.50/month
─────────────────────────────
Total:               ~$2-4/month
```

### High Traffic Site (100K+ visitors/month)
```
Amplify Hosting:     $5-20 (transfer costs)
Domain:              $1/month
Hosted Zone:         $0.50/month
CloudFront:          Varies
─────────────────────────────
Total:               ~$10-30/month
```

### Site + VPS Backend
```
EC2 t3.small:        $15/month
EBS (20GB):          $2/month
Amplify frontend:    $1-2/month
Domain:              $1/month
Hosted Zone:         $0.50/month
─────────────────────────────
Total:               ~$20/month
```

## Cost Alerts

Set up billing alerts to avoid surprises:

```bash
# Create a billing alarm (alert at $10)
aws cloudwatch put-metric-alarm \
  --alarm-name "BillingAlarm" \
  --alarm-description "Alert when charges exceed $10" \
  --metric-name EstimatedCharges \
  --namespace AWS/Billing \
  --statistic Maximum \
  --period 21600 \
  --threshold 10 \
  --comparison-operator GreaterThanThreshold \
  --evaluation-periods 1 \
  --dimensions Name=Currency,Value=USD \
  --alarm-actions <your-sns-topic-arn> \
  --region us-east-1
```

Or set up in console: https://console.aws.amazon.com/billing/home#/budgets

## Next Steps

To see what you're actually spending:
- `obs recipe show view-costs`
