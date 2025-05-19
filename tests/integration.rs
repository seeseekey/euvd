use euvd::apis::configuration::Configuration;
use euvd::apis::default_api;

#[tokio::test()]
async fn get_last_vulnerabilities() {

    // Preparation
    let config = Configuration::default();
    let result = default_api::get_last_vulnerabilities(&config).await;

    // Print result if successful
    if let Ok(response) = &result {
        println!("Response received:");
        for vuln in response {
            println!("• ID: {:?}, Description: {:?}", vuln.id, vuln.description);
        }
    }

    // Asserts
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
}

#[tokio::test()]
async fn get_exploited_vulnerabilities() {

    // Preparation
    let config = Configuration::default();
    let result = default_api::get_exploited_vulnerabilities(&config).await;

    // Print result if successful
    if let Ok(response) = &result {
        println!("Response received:");
        for vuln in response {
            println!("• ID: {:?}, Description: {:?}", vuln.id, vuln.description);
        }
    }

    // Asserts
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
}

#[tokio::test()]
async fn get_critical_vulnerabilities() {

    // Preparation
    let config = Configuration::default();
    let result = default_api::get_critical_vulnerabilities(&config).await;

    // Print result if successful
    if let Ok(response) = &result {
        println!("Response received:");
        for vuln in response {
            println!("• ID: {:?}, Description: {:?}", vuln.id, vuln.description);
        }
    }

    // Asserts
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
}

#[tokio::test]
async fn query_vulnerabilities() {
    
    // Preparation
    let config = Configuration::default();
    let result = default_api::query_vulnerabilities(
        &config,
        None, // from_score
        None, // to_score
        None, // from_epss
        None, // to_epss
        None, // from_date
        None, // to_date
        None, // search
        None, // cvss_version
        None, // cvss_vector
        None, // exploited
        None, // limit
        None, // sort_by
        None, // page
    ).await;

    // Print result if successful
    if let Ok(response) = &result {
        println!("Response received:");

        if let Some(items) = &response.items {
            for vuln in items {
                println!(
                    "• ID: {:?}, Description: {:?}",
                    vuln.id.as_deref().unwrap_or("n/a"),
                    vuln.description.as_deref().unwrap_or("n/a")
                );
            }
        } else {
            println!("No vulnerabilities found in response.");
        }
    }

    // Assert
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
}

#[tokio::test()]
async fn get_by_enisa_id() {

    // Preparation
    let config = Configuration::default();
    let result = default_api::get_by_enisa_id(&config, "EUVD-2024-45012").await;

    // Print result if successful
    if let Ok(vuln) = &result {
        println!("Response received:");
        println!("• ID: {:?}, Description: {:?}", vuln.id, vuln.description);
    }

    // Asserts
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
}

#[tokio::test()]
async fn get_vulnerability_by_id() {

    // Preparation
    let config = Configuration::default();
    let result = default_api::get_vulnerability_by_id(&config, "CVE-2024-0864").await;

    // Print result if successful
    if let Ok(vuln) = &result {
        println!("Response received:");
        println!("• ID: {:?}, Description: {:?}", vuln.id, vuln.description);
    }

    // Asserts
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
}

#[tokio::test()]
async fn get_advisory_by_id() {

    // Preparation
    let config = Configuration::default();
    let result = default_api::get_advisory_by_id(&config, "cisco-sa-ata19x-multi-RDTEqRsy").await;

    // Print result if successful
    if let Ok(vuln) = &result {
        println!("Response received:");
        println!("• ID: {:?}, Description: {:?}", vuln.id, vuln.description);
    }

    // Asserts
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
}