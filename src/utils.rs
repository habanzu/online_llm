use serde::{Deserialize, Serialize};
use reqwest::StatusCode;
use dotenv::dotenv;
use std::env;
use actix_web::{web, Error, HttpResponse};
use reqwest::Client;
use log::info;

#[derive(Serialize, Deserialize)]
pub struct ChatCompletion {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    index: i32,
    logprobs: Option<serde_json::Value>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIRequest {
    model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    
}

impl Message {
    pub fn new<S: Into<String>>(role: S, content: S) -> Message{
        Message{role: role.into(), content: content.into()}
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct GoogleSearchResults {
    items: Vec<Item>,
}

#[derive(Deserialize)]
pub struct Item {
    title: String,
    link: String,
    snippet: String,
}

pub async fn search_google(query: &String) -> GoogleSearchResults {
    dotenv().ok();
    let google_api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY not found in environment");
    let cx = env::var("SEARCH_ENGINE_ID").expect("SEARCH_ENGINE_ID not found in environment");

    let url = format!(
        "https://www.googleapis.com/customsearch/v1?q={}&key={}&cx={}",
        query, google_api_key, cx
    );

    let google_resp = match reqwest::get(&url).await {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => match response.json::<GoogleSearchResults>().await {
                    Ok(google_resp) => {
                        Ok(google_resp)
                    },
                    Err(e) => {
                        Err(format!("Error when deserializing Google response {:?}", e))
                    }
                },
                _ => {
                    Err(String::from("HTTP Request with bad status code."))
                }
            }
        },
        Err(e) => {
            Err(format!("Failed to send request to Google: {:?}", e))
        },
    }.expect("Google Search result is empty.");

    google_resp
}

pub async fn open_ai_response(request: &OpenAIRequest) -> OpenAIResponse{
    dotenv().ok();
    let open_ai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in environment");
    
    let request = web::Json(request);

    let client = Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(open_ai_api_key) 
        .json(&*request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            match resp.status() {
                StatusCode::OK => match resp.json::<OpenAIResponse>().await {
                    Ok(openai_resp) => Ok(openai_resp),
                    Err(_) => {
                        Err("Failed to deserialize OpenAI response:")
                    },
                },
                _ => {
                    Err("Failure with OpenAI response status")
                }
            }
        },
        Err(_) => {
            Err("Bad HTTP Response.")
        }
    }.expect("Failure to retrieve OpenAI answer.")
}

pub async fn return_open_ai_response(request: &OpenAIRequest) -> HttpResponse {
    
    dotenv().ok();
    let open_ai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in environment");
    
    let request = web::Json(request);

    let client = Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(open_ai_api_key) 
        .json(&*request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            match resp.status() {
                StatusCode::OK => match resp.json::<OpenAIResponse>().await {
                    Ok(openai_resp) => HttpResponse::Ok().json(openai_resp),
                    Err(e) => {
                        info!("Failed to deserialize OpenAI response: {:?}", e);
                        HttpResponse::InternalServerError().finish()
                    },
                },
                _ => {
                    let err_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    info!("OpenAI API responded with an error: {}", err_text);
                    HttpResponse::InternalServerError().body(err_text)
                }
            }
        },
        Err(e) => {
            info!("Failed to send request to OpenAI: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub fn message_from_google_search(google_resp: GoogleSearchResults) -> Message {
    let mut message = String::from("Google Search results added to question: \n");

    for item in google_resp.items {
        message.push_str(&format!("Title: {}", item.title));
        message.push_str("\n");
        message.push_str(&format!("Snippet: {}", item.snippet));
        message.push_str("\n");
        message.push_str(&format!("Link: {}", item.link));
        message.push_str("\n");
    }

    Message{role : String::from("system"), content: message}
}
