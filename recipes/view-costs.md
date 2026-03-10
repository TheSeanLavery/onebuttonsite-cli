# View Current Costs

See what you're spending on AWS right now.

## Prerequisites
- AWS CLI configured
- Cost Explorer enabled (first-time setup in console)

## Quick Cost Check

### Via AWS Console (Easiest)
https://console.aws.amazon.com/billing/home#/

### Via CLI

**This month's total:**
```bash
aws ce get-cost-and-usage \
  --time-period Start=$(date -v1d +%Y-%m-%d),End=$(date +%Y-%m-%d) \
  --granularity MONTHLY \
  --metrics "UnblendedCost" \
  --region us-east-1
```

**Breakdown by service:**
```bash
aws ce get-cost-and-usage \
  --time-period Start=$(date -v1d +%Y-%m-%d),End=$(date +%Y-%m-%d) \
  --granularity MONTHLY \
  --metrics "UnblendedCost" \
  --group-by Type=DIMENSION,Key=SERVICE \
  --region us-east-1
```

## Understanding Your Bill

### Amplify Costs
```bash
aws ce get-cost-and-usage \
  --time-period Start=$(date -v1d +%Y-%m-%d),End=$(date +%Y-%m-%d) \
  --granularity MONTHLY \
  --metrics "UnblendedCost" \
  --filter '{"Dimensions":{"Key":"SERVICE","Values":["AWS Amplify"]}}' \
  --region us-east-1
```

### Route53 Costs (Domains + DNS)
```bash
aws ce get-cost-and-usage \
  --time-period Start=$(date -v1d +%Y-%m-%d),End=$(date +%Y-%m-%d) \
  --granularity MONTHLY \
  --metrics "UnblendedCost" \
  --filter '{"Dimensions":{"Key":"SERVICE","Values":["Amazon Route 53"]}}' \
  --region us-east-1
```

### EC2 Costs (if using VPS)
```bash
aws ce get-cost-and-usage \
  --time-period Start=$(date -v1d +%Y-%m-%d),End=$(date +%Y-%m-%d) \
  --granularity MONTHLY \
  --metrics "UnblendedCost" \
  --filter '{"Dimensions":{"Key":"SERVICE","Values":["Amazon Elastic Compute Cloud - Compute"]}}' \
  --region us-east-1
```

## Common Cost Items

| Line Item | What It Is | Typical Cost |
|-----------|------------|--------------|
| `AWS Amplify` | Site hosting | $0-5/mo |
| `Route 53` | DNS + domains | $0.50-2/mo |
| `Route53-Domain-Registration` | Domain purchase | $12-39/yr |
| `Amazon EC2` | VPS/servers | $8-100/mo |
| `CloudFront` | CDN transfer | $0-10/mo |
| `S3` | File storage | $0-5/mo |

## Set Up Budget Alerts

Get notified before you overspend:

**Via Console (Recommended):**
1. Go to https://console.aws.amazon.com/billing/home#/budgets
2. Click "Create budget"
3. Choose "Cost budget"
4. Set amount (e.g., $10/month)
5. Add email notification

**Via CLI:**
```bash
aws budgets create-budget \
  --account-id $(aws sts get-caller-identity --query Account --output text) \
  --budget '{
    "BudgetName": "MonthlyLimit",
    "BudgetLimit": {"Amount": "10", "Unit": "USD"},
    "TimeUnit": "MONTHLY",
    "BudgetType": "COST"
  }' \
  --notifications-with-subscribers '[{
    "Notification": {
      "NotificationType": "ACTUAL",
      "ComparisonOperator": "GREATER_THAN",
      "Threshold": 80
    },
    "Subscribers": [{
      "SubscriptionType": "EMAIL",
      "Address": "you@email.com"
    }]
  }]'
```

## Reducing Costs

### Delete unused resources:
```bash
# List all Amplify apps
obs apps

# Delete unused app
aws amplify delete-app --app-id <app-id> --region us-east-1
```

### Delete unused hosted zones:
```bash
# List zones
obs zones

# Delete zone (must remove all records first except NS and SOA)
aws route53 delete-hosted-zone --id <zone-id>
```

### Stop unused EC2 instances:
```bash
aws ec2 stop-instances --instance-ids <instance-id>
```

## Free Tier Tracking

See how much free tier you've used:
https://console.aws.amazon.com/billing/home#/freetier

## Troubleshooting

**"Cost Explorer not enabled"**
- First-time setup required in console
- Go to https://console.aws.amazon.com/cost-management/home
- Enable Cost Explorer (takes 24 hours to populate)

**"Access Denied"**
- Your IAM user needs `ce:GetCostAndUsage` permission
- Or use the root account
