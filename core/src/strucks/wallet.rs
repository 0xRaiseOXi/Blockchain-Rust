use serde::{Serialize, Deserialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct InsufficientBalance {
    message: String,
}

impl fmt::Display for InsufficientBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub public_key: String,
    balance: HashMap<String, f64>,
}

impl Wallet {
    pub fn new(public_key: String) -> Wallet {
        Self {
            public_key: public_key,
            balance: HashMap::new(),
        }
    }

    pub fn subtraction(&mut self, token: String, amount: f64) -> Result<(), InsufficientBalance> {
        match self.balance.get(&token) {
            Some(value) => {
                let new_value = value - amount;

                if new_value > 0.0 {
                    self.balance.insert(token, new_value);
                    return Ok(());

                } else {
                    return Err(InsufficientBalance {
                        message: "Insufficient Balance".to_string(),
                    });
                }
            }
            None => {
                return Err(InsufficientBalance {
                    message: "Insufficient Balance".to_string(),
                });
            }
        }
    }

    pub fn addition(&mut self, token: String, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
        match self.balance.get(&token) {
            Some(value) => {
                let new_value = value + amount;
                self.balance.insert(token, new_value);
            } 
            None => {
                self.balance.insert(token, amount);
            }
        }
        Ok(())
    }


    pub fn get_balance_token(&self, token: String) -> Option<f64> {
        match self.balance.get(&token) {
            Some(value) => {
                return Some(value.clone());
            } 
            None => {
                return None;
            }
        }
    }

    pub fn get_all_balance(&self) -> Option<HashMap<String, f64>> {
        Some(self.balance.clone())
    }

    pub fn get_json(&self) -> String{
        let json_result = json!({
            "public_key": &self.public_key,
            "balance": &self.balance,
        });
        serde_json::to_string(&json_result).unwrap()
    }
}