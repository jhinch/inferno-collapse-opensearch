use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub shards: Vec<ProfileShard>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileShard {
    pub id: String,
    pub searches: Vec<ProfileSearch>,
    pub aggregations: Vec<ProfileAggregation>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileSearch {
    pub query: Vec<ProfileQuery>,
    pub rewrite_time: usize,

}

#[derive(Debug, Deserialize)]
pub struct ProfileQuery {
    #[serde(rename = "type")]
    pub ty: String,
    pub description: String,
    pub time_in_nanos: usize,
    pub breakdown: HashMap<String, usize>,
    pub children: Vec<ProfileQuery>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileAggregation {}