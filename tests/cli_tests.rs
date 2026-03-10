use std::process::Command;
use serde_json::Value;

fn run_obs(args: &[&str]) -> (bool, String, Option<Value>) {
    let output = Command::new("./target/release/onebuttonsite")
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{}{}", stdout, stderr);
    
    let json: Option<Value> = serde_json::from_str(&stdout).ok();
    
    (output.status.success(), combined, json)
}

// ============================================================
// DOCTOR TESTS (FREE)
// ============================================================

#[test]
fn test_doctor_returns_ok() {
    let (success, _, json) = run_obs(&["doctor"]);
    
    assert!(success, "doctor command should succeed");
    
    let json = json.expect("doctor should return valid JSON");
    assert_eq!(json["ok"], true, "doctor should return ok: true");
    assert!(json["account"].is_string(), "doctor should return account");
    assert!(json["user"].is_string(), "doctor should return user");
}

// ============================================================
// APPS TESTS (FREE)
// ============================================================

#[test]
fn test_apps_returns_list() {
    let (success, _, json) = run_obs(&["apps"]);
    
    assert!(success, "apps command should succeed");
    
    let json = json.expect("apps should return valid JSON");
    assert_eq!(json["ok"], true, "apps should return ok: true");
    assert!(json["apps"].is_array(), "apps should return an array");
}

#[test]
fn test_apps_contains_expected_fields() {
    let (_, _, json) = run_obs(&["apps"]);
    let json = json.expect("apps should return valid JSON");
    
    if let Some(apps) = json["apps"].as_array() {
        if !apps.is_empty() {
            let app = &apps[0];
            assert!(app["appId"].is_string(), "app should have appId");
            assert!(app["name"].is_string(), "app should have name");
            assert!(app["defaultUrl"].is_string(), "app should have defaultUrl");
        }
    }
}

// ============================================================
// CHECK-DOMAIN TESTS (FREE)
// ============================================================

#[test]
fn test_check_domain_available() {
    // Using a random unlikely domain
    let (success, _, json) = run_obs(&["check-domain", "xyzzy123notreal987.com"]);
    
    assert!(success, "check-domain should succeed");
    
    let json = json.expect("check-domain should return valid JSON");
    assert_eq!(json["ok"], true);
    assert!(json["domain"].is_string());
    assert!(json["available"].is_boolean());
    assert!(json["status"].is_string());
}

#[test]
fn test_check_domain_unavailable() {
    // google.com is definitely taken
    let (success, _, json) = run_obs(&["check-domain", "google.com"]);
    
    assert!(success, "check-domain should succeed even for taken domains");
    
    let json = json.expect("check-domain should return valid JSON");
    assert_eq!(json["ok"], true);
    assert_eq!(json["available"], false);
    assert_eq!(json["status"], "UNAVAILABLE");
}

#[test]
fn test_check_domain_unsupported_tld() {
    // .dev is not supported by AWS
    let (success, _, json) = run_obs(&["check-domain", "example.dev"]);
    
    assert!(!success, "check-domain should fail for unsupported TLD");
    
    let json = json.expect("should return JSON error");
    assert_eq!(json["ok"], false);
}

// ============================================================
// ZONES TESTS (FREE)
// ============================================================

#[test]
fn test_zones_returns_list() {
    let (success, _, json) = run_obs(&["zones"]);
    
    assert!(success, "zones command should succeed");
    
    let json = json.expect("zones should return valid JSON");
    assert_eq!(json["ok"], true);
    assert!(json["zones"].is_array(), "zones should return an array");
}

#[test]
fn test_zones_contains_expected_fields() {
    let (_, _, json) = run_obs(&["zones"]);
    let json = json.expect("zones should return valid JSON");
    
    if let Some(zones) = json["zones"].as_array() {
        if !zones.is_empty() {
            let zone = &zones[0];
            assert!(zone["id"].is_string(), "zone should have id");
            assert!(zone["name"].is_string(), "zone should have name");
            assert!(zone["records"].is_number(), "zone should have records count");
        }
    }
}

// ============================================================
// STATUS TESTS (FREE)
// ============================================================

#[test]
fn test_status_nonexistent_app() {
    let (success, _, json) = run_obs(&["status", "nonexistent123"]);
    
    assert!(!success, "status should fail for nonexistent app");
    
    let json = json.expect("should return JSON error");
    assert_eq!(json["ok"], false);
}

#[test]
fn test_status_existing_app() {
    // First get an app ID from the apps list
    let (_, _, apps_json) = run_obs(&["apps"]);
    let apps_json = apps_json.expect("apps should return JSON");
    
    if let Some(apps) = apps_json["apps"].as_array() {
        if !apps.is_empty() {
            let app_id = apps[0]["appId"].as_str().unwrap();
            
            let (success, _, json) = run_obs(&["status", app_id]);
            
            assert!(success, "status should succeed for existing app");
            
            let json = json.expect("status should return JSON");
            assert_eq!(json["ok"], true);
            assert!(json["app"].is_object());
            assert!(json["domains"].is_array());
            assert!(json["statusMeaning"].is_object());
        }
    }
}

// ============================================================
// RECIPE TESTS (FREE - no AWS calls)
// ============================================================

#[test]
fn test_recipe_list() {
    let (success, output, _) = run_obs(&["recipe", "list"]);
    
    assert!(success, "recipe list should succeed");
    assert!(output.contains("OneButtonSite Recipes"));
    assert!(output.contains("create-site"));
    assert!(output.contains("check-domain"));
    assert!(output.contains("estimate-costs"));
}

#[test]
fn test_recipe_show_create_site() {
    let (success, output, _) = run_obs(&["recipe", "show", "create-site"]);
    
    assert!(success, "recipe show create-site should succeed");
    assert!(output.contains("Create and Deploy a New Site"));
    assert!(output.contains("obs create"));
    assert!(output.contains("Prerequisites"));
}

#[test]
fn test_recipe_show_check_domain() {
    let (success, output, _) = run_obs(&["recipe", "show", "check-domain"]);
    
    assert!(success, "recipe show check-domain should succeed");
    assert!(output.contains("Check Domain Availability"));
    assert!(output.contains("obs check-domain"));
}

#[test]
fn test_recipe_show_estimate_costs() {
    let (success, output, _) = run_obs(&["recipe", "show", "estimate-costs"]);
    
    assert!(success, "recipe show estimate-costs should succeed");
    assert!(output.contains("Estimate Costs"));
    assert!(output.contains("Free Tier"));
    assert!(output.contains("Cost by Action"));
}

#[test]
fn test_recipe_show_all_recipes() {
    let recipes = [
        "create-site",
        "check-domain", 
        "buy-domain",
        "add-subdomain",
        "setup-ssl",
        "estimate-costs",
        "view-costs",
        "hosting-frontend",
        "hosting-vps",
        "commands",
    ];
    
    for recipe in recipes {
        let (success, output, _) = run_obs(&["recipe", "show", recipe]);
        assert!(success, "recipe show {} should succeed", recipe);
        assert!(!output.is_empty(), "recipe {} should have content", recipe);
    }
}

#[test]
fn test_recipe_show_nonexistent() {
    let (success, output, _) = run_obs(&["recipe", "show", "nonexistent-recipe"]);
    
    assert!(!success, "recipe show nonexistent should fail");
    assert!(output.contains("not found"));
    assert!(output.contains("Available recipes"));
}

// ============================================================
// HELP TESTS (FREE - no AWS calls)
// ============================================================

#[test]
fn test_help() {
    let (success, output, _) = run_obs(&["--help"]);
    
    assert!(success, "--help should succeed");
    assert!(output.contains("OneButtonSite CLI"));
    assert!(output.contains("doctor"));
    assert!(output.contains("create"));
    assert!(output.contains("recipe"));
}

#[test]
fn test_version() {
    let (success, output, _) = run_obs(&["--version"]);
    
    assert!(success, "--version should succeed");
    assert!(output.contains("obs") || output.contains("0.1.0"));
}

// ============================================================
// ERROR HANDLING TESTS
// ============================================================

#[test]
fn test_create_missing_args() {
    let (success, _, _) = run_obs(&["create"]);
    assert!(!success, "create without args should fail");
}

#[test]
fn test_update_missing_args() {
    let (success, _, _) = run_obs(&["update"]);
    assert!(!success, "update without args should fail");
}

#[test]
fn test_status_missing_args() {
    let (success, _, _) = run_obs(&["status"]);
    assert!(!success, "status without args should fail");
}

#[test]
fn test_subdomain_missing_args() {
    let (success, _, _) = run_obs(&["subdomain"]);
    assert!(!success, "subdomain without args should fail");
}

#[test]
fn test_check_domain_missing_args() {
    let (success, _, _) = run_obs(&["check-domain"]);
    assert!(!success, "check-domain without args should fail");
}

#[test]
fn test_invalid_command() {
    let (success, _, _) = run_obs(&["nonexistent-command"]);
    assert!(!success, "invalid command should fail");
}
