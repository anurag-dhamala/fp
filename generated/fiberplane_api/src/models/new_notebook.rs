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
pub struct NewNotebook {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "timeRange")]
    pub time_range: Box<crate::models::TimeRange>,
    #[serde(rename = "cells")]
    pub cells: Vec<crate::models::Cell>,
    #[serde(rename = "dataSources", skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<::std::collections::HashMap<String, crate::models::NotebookDataSource>>,
}

impl NewNotebook {
    pub fn new(title: String, time_range: crate::models::TimeRange, cells: Vec<crate::models::Cell>) -> NewNotebook {
        NewNotebook {
            title,
            time_range: Box::new(time_range),
            cells,
            data_sources: None,
        }
    }
}

