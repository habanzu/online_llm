use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use actix_web::HttpRequest;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub examples: String,
    pub first_instruction: String,
    pub second_instruction: String,
    pub final_instruction: String
}

impl Config{
    pub fn new(file_path: String) -> Config{
        let file_content = fs::read_to_string(file_path).expect("Could not read file.");
        serde_json::from_str(&file_content).expect("Could not parse config")
    }
}

pub fn authorize(req: HttpRequest) -> bool{
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found in environment");
    
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str.trim_start_matches("Bearer ");

                let valid_token = api_key;
                if token == valid_token {
                    return true
                }
            }
        }
    }
    return false
}