/*
 * Fiberplane API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
}

impl Metric {
    pub fn new(name: String, labels: ::std::collections::HashMap<String, String>) -> Metric {
        Metric {
            name,
            labels,
        }
    }
}

