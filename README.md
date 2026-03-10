# OneButtonSite CLI

Deploy static sites to AWS Amplify with one command. A fast, minimal CLI written in Rust.

```bash
obs create my-site ./dist
# Site live at https://main.abc123.amplifyapp.com in ~30 seconds
```

## Features

- **Fast** — 623KB binary, ~50ms startup
- **Simple** — One command to deploy
- **No Dependencies** — Just needs AWS CLI
- **Recipes** — Built-in workflow guides for AI agents and humans
- **Cross-platform** — macOS, Linux, Windows

## Installation

### From Source

```bash
git clone https://github.com/seanlavery/onebuttonsite-cli.git
cd onebuttonsite-cli
cargo install --path .
```

### Pre-built Binaries

Coming soon via GitHub Releases.

## Prerequisites

- AWS CLI configured (`aws configure`)
- AWS account with Amplify access

## Quick Start

```bash
# Check AWS credentials
obs doctor

# Create and deploy a site
obs create my-site ./dist

# Update an existing site
obs update <app-id> ./dist

# Add a custom domain
obs subdomain <app-id> blog example.com
```

## Commands

| Command | Description |
|---------|-------------|
| `obs doctor` | Check AWS credentials |
| `obs apps` | List all Amplify apps |
| `obs create <name> <dir>` | Create and deploy a new app |
| `obs update <app-id> <dir>` | Update an existing app |
| `obs status <app-id>` | Check domain/SSL status |
| `obs subdomain <app-id> <prefix> <domain>` | Add subdomain |
| `obs domain <app-id> <domain>` | Add root domain with www |
| `obs zones` | List Route53 hosted zones |
| `obs dns <zone-id> <type> <name> <value>` | Create DNS record |
| `obs check-domain <domain>` | Check domain availability |
| `obs recipe list` | List all recipes |
| `obs recipe show <name>` | Show a recipe |

## Recipes

Built-in workflow guides that help you (and AI agents) accomplish common tasks:

```bash
obs recipe list                    # See all recipes
obs recipe show create-site        # How to deploy a site
obs recipe show estimate-costs     # Understand AWS costs
```

### Available Recipes

| Recipe | Description |
|--------|-------------|
| `create-site` | Create and deploy a new static site |
| `check-domain` | Check if a domain is available |
| `buy-domain` | Purchase a domain through AWS |
| `add-subdomain` | Add a subdomain to an existing site |
| `setup-ssl` | Configure SSL/HTTPS |
| `estimate-costs` | Understand costs before taking action |
| `view-costs` | View current AWS spending |
| `hosting-frontend` | Host a frontend on Amplify |
| `hosting-vps` | Set up a VPS on EC2 |
| `commands` | Quick command reference |

## Examples

### Deploy a React App

```bash
# Build your app
npm run build

# Deploy
obs create my-react-app ./build

# Output:
# {
#   "ok": true,
#   "appId": "d1abc123xyz",
#   "url": "https://main.d1abc123xyz.amplifyapp.com"
# }
```

### Add a Custom Domain

```bash
# Add subdomain to your app
obs subdomain d1abc123xyz blog example.com

# Get the DNS record from the output
# Add it to Route53
obs dns Z1234567890 CNAME blog.example.com d123.cloudfront.net

# Check status (wait for AVAILABLE)
obs status d1abc123xyz
```

### Check Domain Availability

```bash
obs check-domain mycoolstartup.com

# {
#   "ok": true,
#   "domain": "mycoolstartup.com",
#   "available": true,
#   "price": "~$12-15/year for .com"
# }
```

## Output Format

All commands return JSON for easy parsing:

```json
{
  "ok": true,
  "appId": "d1abc123xyz",
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

## Costs

Most operations are free or very cheap:

| Operation | Cost |
|-----------|------|
| Deploy site | FREE (within free tier) |
| Check domain | FREE |
| Add subdomain | FREE |
| SSL certificate | FREE |
| Hosting (small site) | ~$0-2/month |
| Domain registration | ~$12/year (.com) |

See `obs recipe show estimate-costs` for detailed breakdown.

## Testing

```bash
cargo test
```

24 integration tests covering all non-billable operations.

## For AI Agents

This CLI is designed to be used by AI agents (like Claude in Cursor). The recipes provide step-by-step instructions that agents can follow:

```bash
# Agent reads the recipe
obs recipe show create-site

# Agent follows the steps
obs doctor
obs create my-site ./dist
obs status <app-id>
```

## License

MIT

## Contributing

PRs welcome! Please ensure tests pass (`cargo test`).
