use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Dfa {
    pub start_state: String,
    pub keywords: Vec<String>,
    pub word_logical_operators: Vec<String>,
    pub word_arithmetic_operators: Vec<String>,
    pub final_states: HashMap<String, String>,
    pub transitions: HashMap<String, HashMap<String, String>>,
}

impl Dfa {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = std::fs::read_to_string(path)?;
        let dfa: Dfa = serde_json::from_str(&file_content)?;
        Ok(dfa)
    }
}
