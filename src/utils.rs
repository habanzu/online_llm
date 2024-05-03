use serde::{Deserialize, Serialize};
use reqwest::StatusCode;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use dotenv::dotenv;
use std::env;
use actix_web::{web, HttpResponse, HttpRequest};
use reqwest::Client;
use log::info;
use serde_json::json;

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
    pub model: String,
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
pub struct SerperResults {
    organic: Option<Vec<Item>>,
    knowledgeGraph: Option<String>
}

#[derive(Deserialize)]
pub struct Item {
    title: String,
    link: String,
    snippet: String,
    date: Option<String>,
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

pub async fn search_serper_google(query: &String) -> String{
    let client = reqwest::Client::new();
    let url = "https://google.serper.dev/search";

    // Creating the payload as a JSON object
    let payload = json!({
        "q": query
    });

    // Creating and setting headers
    let mut headers = HeaderMap::new();
    headers.insert("X-API-KEY", HeaderValue::from_static("567a754bf49e917366276b03682ca0e2b5e58328"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Making the POST request
    let res = client.post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await.expect("Serper Call failed.");

    // Reading the response
    let data = res.text().await.expect("Received data is empty.");
    let result:Result<SerperResults, serde_json::Error> = serde_json::from_str(&data);
    let result = result.expect("Unable to read serper result.");

    let mut message = String::from("Google Search results added to question: \n");

    

    if let Some(items) = result.organic {
        for item in items.iter().rev() {
            message.push_str(&format!("Title: {}", item.title));
            message.push_str("\n");
            message.push_str(&format!("Snippet: {}", item.snippet));
            message.push_str("\n");
            if let Some(date) = &item.date {
                message.push_str(&format!("Date: {}", date));
                message.push_str("\n");
            }
            // message.push_str(&format!("Link: {}", item.link));
            // message.push_str("\n");
        }
    };

    if let Some(graph) = result.knowledgeGraph {
        message.push_str("\nKnowledge Graph\n");
        message.push_str(&graph);
        message.push_str("\n");
    }

    message

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