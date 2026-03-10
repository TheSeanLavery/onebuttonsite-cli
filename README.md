# OneButtonSite CLI

Deploy static sites to AWS Amplify with one command.

## Installation

### From Source (Rust)

```bash
cargo install --path .
```

### Pre-built Binaries

Download from [Releases](https://github.com/seanlavery/onebuttonsite-cli/releases).

## Prerequisites

- AWS CLI configured with credentials (`aws configure`)
- `zip` and `curl` (included on macOS/Linux)

## Usage

```bash
# Check AWS credentials
obs doctor

# Create a new site
obs create my-site ./dist

# Update an existing site
obs update <app-id> ./dist

# List all apps
obs apps

# Check domain availability
obs check-domain mycoolsite.com

# Add a custom domain
obs domain <app-id> example.com

# Add a subdomain
obs subdomain <app-id> blog example.com

# Check status
obs status <app-id>

# List DNS zones
obs zones

# Create DNS record
obs dns <zone-id> CNAME www.example.com target.cloudfront.net
```

## Commands

| Command | Description |
|---------|-------------|
| `doctor` | Check AWS credentials |
| `apps` | List all Amplify apps |
| `create <name> <dir>` | Create and deploy a new app |
| `update <app-id> <dir>` | Update an existing app |
| `status <app-id>` | Check domain/SSL status |
| `subdomain <app-id> <prefix> <domain>` | Add subdomain |
| `domain <app-id> <domain>` | Add root domain with www |
| `zones` | List Route53 hosted zones |
| `dns <zone-id> <type> <name> <value>` | Create DNS record |
| `check-domain <domain>` | Check domain availability |
| `help` | Show workflow examples |

## License

MIT
