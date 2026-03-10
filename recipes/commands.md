# Command Reference

Quick reference for all CLI commands.

## Deployment Commands

### obs doctor
Check AWS credentials are configured.
```bash
obs doctor
```

### obs create
Create and deploy a new site.
```bash
obs create <name> <directory>
obs create my-site ./dist
```

### obs update
Update an existing site.
```bash
obs update <app-id> <directory>
obs update d1abc123 ./dist
```

### obs apps
List all Amplify apps.
```bash
obs apps
```

### obs status
Check app and domain status.
```bash
obs status <app-id>
obs status d1abc123
```

## Domain Commands

### obs check-domain
Check if a domain is available.
```bash
obs check-domain <domain>
obs check-domain mycoolsite.com
```

### obs domain
Add a root domain (+ www) to an app.
```bash
obs domain <app-id> <domain>
obs domain d1abc123 example.com
```

### obs subdomain
Add a subdomain to an app.
```bash
obs subdomain <app-id> <prefix> <domain>
obs subdomain d1abc123 blog example.com
```

## DNS Commands

### obs zones
List Route53 hosted zones.
```bash
obs zones
```

### obs dns
Create or update a DNS record.
```bash
obs dns <zone-id> <type> <name> <value> [--ttl 300]
obs dns Z1234 CNAME www.example.com d123.cloudfront.net
obs dns Z1234 A api.example.com 1.2.3.4
```

## Help Commands

### obs help
Show all commands.
```bash
obs --help
```

### obs recipe
Show a recipe (workflow guide).
```bash
obs recipe show <recipe-name>
obs recipe show create-site
obs recipe list
```

## Output Format

All commands output JSON:
```json
{
  "ok": true,
  "appId": "d1abc123",
  "url": "https://..."
}
```

On error:
```json
{
  "ok": false,
  "error": "Description of what went wrong"
}
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `AWS_ACCESS_KEY_ID` | AWS access key |
| `AWS_SECRET_ACCESS_KEY` | AWS secret key |
| `AWS_REGION` | Default region (default: us-east-1) |
| `AWS_PROFILE` | Use a named profile from ~/.aws/credentials |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (see JSON output for details) |
