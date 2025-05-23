/*
 * EUVD API
 *
 * API for querying recent vulnerabilities from the ENISA EUVD database.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvisoryEnisaIdAdvisoriesInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "enisaId", skip_serializing_if = "Option::is_none")]
    pub enisa_id: Option<Box<models::NestedVulnerability>>,
}

impl AdvisoryEnisaIdAdvisoriesInner {
    pub fn new() -> AdvisoryEnisaIdAdvisoriesInner {
        AdvisoryEnisaIdAdvisoriesInner {
            id: None,
            enisa_id: None,
        }
    }
}

