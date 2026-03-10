use clap::{Parser, Subcommand};
use colored::*;
use serde_json::{json, Value};
use std::process::{Command, Stdio};
use std::path::Path;
use std::fs;

const REGION: &str = "us-east-1";

#[derive(Parser)]
#[command(name = "obs")]
#[command(about = "OneButtonSite CLI — deploy static sites to AWS Amplify", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check AWS credentials and connectivity
    Doctor,
    
    /// List all Amplify apps with their domains
    Apps,
    
    /// Create a NEW Amplify app and deploy
    Create {
        /// Name for the new app
        name: String,
        /// Directory containing site files
        directory: String,
    },
    
    /// Deploy updates to an EXISTING Amplify app
    Update {
        /// App ID (from 'obs apps')
        app_id: String,
        /// Directory containing site files
        directory: String,
    },
    
    /// Check deployment and domain/SSL status
    Status {
        /// App ID to check
        app_id: String,
    },
    
    /// Add a subdomain to an app
    Subdomain {
        /// App ID
        app_id: String,
        /// Subdomain prefix (e.g., 'blog')
        prefix: String,
        /// Domain name (e.g., 'example.com')
        domain: String,
    },
    
    /// Add a root domain (with www) to an app
    Domain {
        /// App ID
        app_id: String,
        /// Domain name (e.g., 'example.com')
        domain: String,
    },
    
    /// List Route53 hosted zones
    Zones,
    
    /// Create or update a DNS record
    Dns {
        /// Hosted zone ID
        zone_id: String,
        /// Record type (A, CNAME, etc.)
        record_type: String,
        /// Record name
        name: String,
        /// Record value
        value: String,
        /// TTL in seconds
        #[arg(long, default_value = "300")]
        ttl: u32,
    },
    
    /// Check if a domain is available for purchase
    CheckDomain {
        /// Domain name to check
        domain: String,
    },
    
    /// Show common workflows and examples
    Help,
}

fn aws(args: &[&str]) -> Result<Value, String> {
    let output = Command::new("aws")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute aws command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.trim().to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return Ok(Value::Null);
    }
    
    serde_json::from_str(stdout.trim())
        .map_err(|_| stdout.trim().to_string())
        .or_else(|s| Ok(Value::String(s)))
}

fn output_json(data: &Value) {
    println!("{}", serde_json::to_string_pretty(data).unwrap());
}

fn success(data: Value) {
    let mut obj = serde_json::Map::new();
    obj.insert("ok".to_string(), Value::Bool(true));
    if let Value::Object(map) = data {
        for (k, v) in map {
            obj.insert(k, v);
        }
    }
    output_json(&Value::Object(obj));
}

fn fail(error: &str) {
    eprintln!("{} {}", "Error:".red().bold(), error);
    output_json(&json!({ "ok": false, "error": error }));
    std::process::exit(1);
}

fn status_msg(msg: &str) {
    eprintln!("{}", msg.cyan());
}

fn deploy_to_app(app_id: &str, dir: &Path) -> Result<(), String> {
    let zip_path = format!("/tmp/deploy-{}-{}.zip", app_id, std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());

    // Create zip
    let zip_output = Command::new("zip")
        .args(["-r", &zip_path, ".", "-x", "*.DS_Store"])
        .current_dir(dir)
        .output()
        .map_err(|e| format!("Failed to create zip: {}", e))?;

    if !zip_output.status.success() {
        return Err("Failed to create zip file".to_string());
    }

    // Create deployment
    let deploy_result = aws(&[
        "amplify", "create-deployment",
        "--app-id", app_id,
        "--branch-name", "main",
        "--region", REGION,
        "--output", "json",
    ])?;

    let upload_url = deploy_result["zipUploadUrl"]
        .as_str()
        .ok_or("No upload URL returned")?;
    let job_id = deploy_result["jobId"]
        .as_str()
        .ok_or("No job ID returned")?;

    // Upload zip
    status_msg("Uploading...");
    let curl_output = Command::new("curl")
        .args(["-s", "-X", "PUT", "-H", "Content-Type: application/zip", "-T", &zip_path, upload_url])
        .output()
        .map_err(|e| format!("Failed to upload: {}", e))?;

    if !curl_output.status.success() {
        return Err("Failed to upload zip".to_string());
    }

    // Start deployment
    aws(&[
        "amplify", "start-deployment",
        "--app-id", app_id,
        "--branch-name", "main",
        "--job-id", job_id,
        "--region", REGION,
    ])?;

    // Wait for deployment
    status_msg("Waiting for deployment...");
    for _ in 0..30 {
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        let status = aws(&[
            "amplify", "get-job",
            "--app-id", app_id,
            "--branch-name", "main",
            "--job-id", job_id,
            "--region", REGION,
            "--output", "json",
        ])?;

        if let Some(job_status) = status["job"]["summary"]["status"].as_str() {
            match job_status {
                "SUCCEED" => {
                    status_msg("Deployment complete!");
                    // Clean up zip
                    let _ = fs::remove_file(&zip_path);
                    return Ok(());
                }
                "FAILED" => {
                    let _ = fs::remove_file(&zip_path);
                    return Err("Deployment failed".to_string());
                }
                _ => continue,
            }
        }
    }

    let _ = fs::remove_file(&zip_path);
    Err("Deployment timed out".to_string())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Doctor => {
            match aws(&["sts", "get-caller-identity", "--output", "json"]) {
                Ok(data) => {
                    let account = data["Account"].as_str().unwrap_or("unknown");
                    let arn = data["Arn"].as_str().unwrap_or("unknown");
                    let user = arn.split('/').last().unwrap_or("unknown");
                    success(json!({
                        "account": account,
                        "user": user,
                        "message": "AWS credentials are configured correctly"
                    }));
                }
                Err(e) => fail(&format!("AWS credentials not configured: {}", e)),
            }
        }

        Commands::Apps => {
            match aws(&["amplify", "list-apps", "--region", REGION, "--output", "json"]) {
                Ok(data) => {
                    let mut apps = Vec::new();
                    if let Some(app_list) = data["apps"].as_array() {
                        for app in app_list {
                            let app_id = app["appId"].as_str().unwrap_or("");
                            let name = app["name"].as_str().unwrap_or("");
                            
                            // Get domains for each app
                            let domains = aws(&[
                                "amplify", "list-domain-associations",
                                "--app-id", app_id,
                                "--region", REGION,
                                "--output", "json",
                            ]).ok();

                            let custom_domains: Vec<Value> = domains
                                .and_then(|d| d["domainAssociations"].as_array().cloned())
                                .unwrap_or_default()
                                .iter()
                                .map(|d| {
                                    let subdomains: Vec<String> = d["subDomains"]
                                        .as_array()
                                        .map(|subs| subs.iter().filter_map(|s| {
                                            s["subDomainSetting"]["prefix"].as_str()
                                                .map(|p| if p.is_empty() { "(root)".to_string() } else { p.to_string() })
                                        }).collect())
                                        .unwrap_or_default();
                                    
                                    json!({
                                        "domain": d["domainName"].as_str().unwrap_or(""),
                                        "status": d["domainStatus"].as_str().unwrap_or(""),
                                        "subdomains": subdomains
                                    })
                                })
                                .collect();

                            apps.push(json!({
                                "name": name,
                                "appId": app_id,
                                "defaultUrl": format!("https://main.{}.amplifyapp.com", app_id),
                                "customDomains": custom_domains
                            }));
                        }
                    }
                    success(json!({ "apps": apps }));
                }
                Err(e) => fail(&format!("Failed to list apps: {}", e)),
            }
        }

        Commands::Create { name, directory } => {
            let dir = Path::new(&directory);
            if !dir.exists() {
                fail(&format!("Directory not found: {}", directory));
            }

            status_msg(&format!("Creating new Amplify app: {}...", name));
            
            match aws(&["amplify", "create-app", "--name", &name, "--region", REGION, "--output", "json"]) {
                Ok(data) => {
                    let app_id = data["app"]["appId"].as_str().unwrap_or("");
                    
                    status_msg("Creating main branch...");
                    let _ = aws(&[
                        "amplify", "create-branch",
                        "--app-id", app_id,
                        "--branch-name", "main",
                        "--region", REGION,
                    ]);

                    status_msg("Deploying...");
                    if let Err(e) = deploy_to_app(app_id, dir) {
                        fail(&e);
                    }

                    success(json!({
                        "appId": app_id,
                        "name": name,
                        "url": format!("https://main.{}.amplifyapp.com", app_id),
                        "console": format!("https://console.aws.amazon.com/amplify/home#/{}", app_id),
                        "hint": format!("To update this app later: obs update {} {}", app_id, directory)
                    }));
                }
                Err(e) => fail(&format!("Failed to create app: {}", e)),
            }
        }

        Commands::Update { app_id, directory } => {
            let dir = Path::new(&directory);
            if !dir.exists() {
                fail(&format!("Directory not found: {}", directory));
            }

            // Verify app exists
            match aws(&["amplify", "get-app", "--app-id", &app_id, "--region", REGION, "--output", "json"]) {
                Ok(data) => {
                    let name = data["app"]["name"].as_str().unwrap_or("unknown");
                    status_msg(&format!("Updating app {} ({})...", name, app_id));
                    
                    if let Err(e) = deploy_to_app(&app_id, dir) {
                        fail(&e);
                    }

                    // Get custom domain if any
                    let domains = aws(&[
                        "amplify", "list-domain-associations",
                        "--app-id", &app_id,
                        "--region", REGION,
                        "--output", "json",
                    ]).ok();

                    let custom_domain = domains
                        .and_then(|d| d["domainAssociations"][0]["domainName"].as_str().map(String::from));

                    let url = custom_domain
                        .map(|d| format!("https://{}", d))
                        .unwrap_or_else(|| format!("https://main.{}.amplifyapp.com", app_id));

                    success(json!({
                        "appId": app_id,
                        "name": name,
                        "url": url
                    }));
                }
                Err(_) => fail(&format!("App not found: {}. Run 'obs apps' to see available apps.", app_id)),
            }
        }

        Commands::Status { app_id } => {
            match aws(&["amplify", "get-app", "--app-id", &app_id, "--region", REGION, "--output", "json"]) {
                Ok(app_data) => {
                    let name = app_data["app"]["name"].as_str().unwrap_or("unknown");
                    
                    let domains = aws(&[
                        "amplify", "list-domain-associations",
                        "--app-id", &app_id,
                        "--region", REGION,
                        "--output", "json",
                    ]).ok();

                    let domain_status: Vec<Value> = domains
                        .and_then(|d| d["domainAssociations"].as_array().cloned())
                        .unwrap_or_default()
                        .iter()
                        .map(|d| {
                            let subdomains: Vec<Value> = d["subDomains"]
                                .as_array()
                                .map(|subs| subs.iter().map(|s| {
                                    let prefix = s["subDomainSetting"]["prefix"].as_str().unwrap_or("");
                                    json!({
                                        "prefix": if prefix.is_empty() { "(root)" } else { prefix },
                                        "verified": s["verified"].as_bool().unwrap_or(false),
                                        "dnsRecord": s["dnsRecord"].as_str().unwrap_or("")
                                    })
                                }).collect())
                                .unwrap_or_default();
                            
                            json!({
                                "domain": d["domainName"].as_str().unwrap_or(""),
                                "status": d["domainStatus"].as_str().unwrap_or(""),
                                "subdomains": subdomains,
                                "sslValidation": d["certificateVerificationDNSRecord"]
                            })
                        })
                        .collect();

                    success(json!({
                        "app": {
                            "name": name,
                            "appId": app_id,
                            "defaultUrl": format!("https://main.{}.amplifyapp.com", app_id)
                        },
                        "domains": domain_status,
                        "statusMeaning": {
                            "AVAILABLE": "Ready to use",
                            "PENDING_VERIFICATION": "Waiting for SSL certificate - add the validation CNAME record",
                            "AWAITING_APP_CNAME": "SSL issued - waiting for DNS records to propagate",
                            "PENDING_DEPLOYMENT": "Domain configured - waiting for deployment",
                            "CREATING": "Domain being configured",
                            "REQUESTING_CERTIFICATE": "SSL certificate being requested"
                        }
                    }));
                }
                Err(_) => fail(&format!("App not found: {}", app_id)),
            }
        }

        Commands::Subdomain { app_id, prefix, domain } => {
            status_msg(&format!("Adding {}.{} to app {}...", prefix, domain, app_id));
            
            match aws(&[
                "amplify", "create-domain-association",
                "--app-id", &app_id,
                "--domain-name", &domain,
                "--sub-domain-settings", &format!("prefix={},branchName=main", prefix),
                "--region", REGION,
                "--output", "json",
            ]) {
                Ok(data) => {
                    let dns_record = data["domainAssociation"]["subDomains"][0]["dnsRecord"]
                        .as_str().unwrap_or("");
                    let ssl_validation = data["domainAssociation"]["certificateVerificationDNSRecord"]
                        .as_str();
                    let status = data["domainAssociation"]["domainStatus"]
                        .as_str().unwrap_or("unknown");

                    let mut next_steps = vec![
                        format!("Add this CNAME to Route53: {}", dns_record),
                    ];
                    if let Some(ssl) = ssl_validation {
                        next_steps.push(format!("Add SSL validation CNAME: {}", ssl));
                    }
                    next_steps.push(format!("Check status with: obs status {}", app_id));

                    success(json!({
                        "subdomain": format!("{}.{}", prefix, domain),
                        "status": status,
                        "dnsRecordNeeded": dns_record,
                        "sslValidationNeeded": ssl_validation,
                        "nextSteps": next_steps
                    }));
                }
                Err(e) => fail(&format!("Failed to add subdomain: {}", e)),
            }
        }

        Commands::Domain { app_id, domain } => {
            status_msg(&format!("Adding {} and www.{} to app {}...", domain, domain, app_id));
            
            match aws(&[
                "amplify", "create-domain-association",
                "--app-id", &app_id,
                "--domain-name", &domain,
                "--sub-domain-settings", "prefix=,branchName=main", "prefix=www,branchName=main",
                "--region", REGION,
                "--output", "json",
            ]) {
                Ok(data) => {
                    let subdomains = data["domainAssociation"]["subDomains"].as_array();
                    let dns_records: Vec<&str> = subdomains
                        .map(|subs| subs.iter().filter_map(|s| s["dnsRecord"].as_str()).collect())
                        .unwrap_or_default();
                    let ssl_validation = data["domainAssociation"]["certificateVerificationDNSRecord"]
                        .as_str();
                    let status = data["domainAssociation"]["domainStatus"]
                        .as_str().unwrap_or("unknown");

                    success(json!({
                        "domain": domain,
                        "status": status,
                        "dnsRecordsNeeded": dns_records,
                        "sslValidationNeeded": ssl_validation,
                        "nextSteps": [
                            format!("1. Add SSL validation CNAME to Route53: {:?}", ssl_validation),
                            format!("2. Add ALIAS record for {} pointing to CloudFront", domain),
                            format!("3. Add CNAME record for www.{}", domain),
                            format!("4. Check status with: obs status {}", app_id)
                        ]
                    }));
                }
                Err(e) => fail(&format!("Failed to add domain: {}", e)),
            }
        }

        Commands::Zones => {
            match aws(&["route53", "list-hosted-zones", "--output", "json"]) {
                Ok(data) => {
                    let zones: Vec<Value> = data["HostedZones"]
                        .as_array()
                        .map(|zones| zones.iter().map(|z| {
                            json!({
                                "name": z["Name"].as_str().unwrap_or("").trim_end_matches('.'),
                                "id": z["Id"].as_str().unwrap_or("").replace("/hostedzone/", ""),
                                "records": z["ResourceRecordSetCount"]
                            })
                        }).collect())
                        .unwrap_or_default();

                    success(json!({ "zones": zones }));
                }
                Err(e) => fail(&format!("Failed to list zones: {}", e)),
            }
        }

        Commands::Dns { zone_id, record_type, name, value, ttl } => {
            let change_batch = json!({
                "Changes": [{
                    "Action": "UPSERT",
                    "ResourceRecordSet": {
                        "Name": name,
                        "Type": record_type.to_uppercase(),
                        "TTL": ttl,
                        "ResourceRecords": [{ "Value": value }]
                    }
                }]
            });

            match aws(&[
                "route53", "change-resource-record-sets",
                "--hosted-zone-id", &zone_id,
                "--change-batch", &change_batch.to_string(),
                "--output", "json",
            ]) {
                Ok(_) => {
                    success(json!({
                        "record": {
                            "type": record_type.to_uppercase(),
                            "name": name,
                            "value": value
                        }
                    }));
                }
                Err(e) => fail(&format!("Failed to create DNS record: {}", e)),
            }
        }

        Commands::CheckDomain { domain } => {
            match aws(&[
                "route53domains", "check-domain-availability",
                "--domain-name", &domain,
                "--region", REGION,
                "--output", "json",
            ]) {
                Ok(data) => {
                    let availability = data["Availability"].as_str().unwrap_or("UNKNOWN");
                    let available = availability == "AVAILABLE";
                    
                    success(json!({
                        "domain": domain,
                        "available": available,
                        "status": availability,
                        "price": if available { Some("~$12-15/year for .com") } else { None::<&str> }
                    }));
                }
                Err(e) => fail(&format!("Failed to check domain: {}", e)),
            }
        }

        Commands::Help => {
            println!(r#"
╔══════════════════════════════════════════════════════════════════╗
║                    OneButtonSite CLI Workflows                   ║
╚══════════════════════════════════════════════════════════════════╝

┌─ CREATE A NEW SITE ─────────────────────────────────────────────┐
│                                                                  │
│   1. Create the site:                                           │
│      obs create my-site ./sites/my-site                         │
│                                                                  │
│   2. Note the appId in the output (e.g., abc123xyz)             │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘

┌─ UPDATE AN EXISTING SITE ───────────────────────────────────────┐
│                                                                  │
│   1. Find your app ID:                                          │
│      obs apps                                                   │
│                                                                  │
│   2. Deploy updates:                                            │
│      obs update <app-id> ./sites/my-site                        │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘

┌─ ADD A CUSTOM DOMAIN ───────────────────────────────────────────┐
│                                                                  │
│   For root domain (example.com + www):                          │
│      obs domain <app-id> example.com                            │
│                                                                  │
│   For subdomain (blog.example.com):                             │
│      obs subdomain <app-id> blog example.com                    │
│                                                                  │
│   Then add DNS records shown in the output to Route53.          │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘

┌─ CHECK DOMAIN AVAILABILITY ─────────────────────────────────────┐
│                                                                  │
│   obs check-domain mycoolsite.com                               │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘

┌─ CHECK STATUS ──────────────────────────────────────────────────┐
│                                                                  │
│   Check domain/SSL status:                                      │
│      obs status <app-id>                                        │
│                                                                  │
│   Status meanings:                                              │
│   • AVAILABLE = Ready to use                                    │
│   • PENDING_VERIFICATION = Add SSL validation CNAME             │
│   • AWAITING_APP_CNAME = Waiting for DNS propagation            │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
"#);
        }
    }
}
