use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, Local};
use std::collections::HashMap;

// struct State {
    
// } 


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from: Option<String>,
    pub function: Option<String>,
    pub params: Option<HashMap<String, String>>,
    pub timestamp: Option<u64>,
    pub signature: Option<String>,
    pub fee: Option<f64>,
    pub state: Option<String>,
    pub hash: Option<String>,
    pub blockchain_params: Option<HashMap<String, String>>,
}

impl Transaction {
    pub fn calculate_hash(&mut self) {
        let transaction_data = self.build_json("hash".to_string());

        let mut hasher = Sha256::new();
        hasher.update(transaction_data);
        let result = hasher.finalize();

        let hash = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        self.hash = Some(hash.clone());
    }

    pub fn set_state(&mut self, state: String) {
        self.state = Some(state)
    }

    pub fn build_json(&self, type_build: String) -> String {
        let json_result;

        if type_build == "hash" {
            json_result = json!({
                "from": &self.from.clone().unwrap_or_default(),
                "function": &self.function.clone().unwrap_or_default(),
                "params": &self.params.clone().unwrap_or_default(),
                "timestamp": self.timestamp.unwrap_or_default(),
                "signature": self.signature.clone().unwrap_or_default(),
                "fee": self.fee.unwrap_or_default(),
            });
            return serde_json::to_string(&json_result).unwrap();

        } else if type_build == "print" {
            json_result = json!({
                "from": &self.from.clone().unwrap_or_default(),
                "function": &self.function.clone().unwrap_or_default(),
                "params": &self.params.clone().unwrap_or_default(),
                "timestamp": self.timestamp.unwrap_or_default(),
                "signature": self.signature.clone().unwrap_or_default(),
                "fee": self.fee.unwrap_or_default(),
                "hash": self.hash.clone().unwrap_or_default(),
            });
            return serde_json::to_string_pretty(&json_result).unwrap();
            
        }  else if type_build == "signature_1" {
            json_result = json!({
                "from": &self.from.clone().unwrap_or_default(),
                "function": &self.function.clone().unwrap_or_default(),
                "params": &self.params.clone().unwrap_or_default(),
                "timestamp": self.timestamp.unwrap_or_default(),
                "signature": self.signature.clone().unwrap_or_default(),
                "fee": self.fee.unwrap_or_default(),
            });
            return serde_json::to_string(&json_result).unwrap();
        } else {
            return "".to_string();
        }
    }

    pub fn current_time_millis() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_millis())
    }

    pub fn format_unix_millis(unix_millis: u64) -> String {
        let dt = UNIX_EPOCH + std::time::Duration::from_millis(unix_millis);
        let datetime_utc: DateTime<Utc> = dt.into();
        let datetime_local: DateTime<Local> = dt.into();

        let formatted_utc = datetime_utc.format("%Y-%m-%d %H:%M:%S.%3f UTC");
        let formatted_local = datetime_local.format("%Y-%m-%d %H:%M:%S.%3f Local");
    
        format!("UTC: {}, Local: {}", formatted_utc, formatted_local)
    }

    pub fn set_timestamp_unix(&mut self) {
        self.timestamp = Some(Self::current_time_millis());
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = Some(timestamp);
    }

    pub fn set_fee(&mut self, fee: f64) {
        self.fee = Some(fee);
    }

    pub fn get_hash(&self) -> Option<String> {
        self.hash.clone()
    }

    pub fn get_timestamp(&self) -> Option<u64> {
        self.timestamp
    }

    pub fn get_function(&self) -> String {
        self.function.clone().unwrap_or_default()
    }

    pub fn get_params(&self) -> HashMap<String, String>{
        self.params.clone().unwrap_or_default()
    }
}

