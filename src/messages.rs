use serde::{Deserialize, Serialize};
use reqwest::{StatusCode, Error};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use dotenv::dotenv;
use std::env;
use actix_web::{web, HttpResponse};
use reqwest::Client;
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

pub async fn open_ai_response(request: &OpenAIRequest) -> Result<OpenAIResponse, &str>{
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
                        Err("Failed to deserialize OpenAI response.")
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
    }
}

pub async fn return_open_ai_response(request: &OpenAIRequest) -> HttpResponse {
    let response  = open_ai_response(request).await;
    HttpResponse::Ok().json(response)
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
    dotenv().ok();
    let serper_api_key = env::var("SERPER_API_KEY").expect("SERPER_API_KEY not found in environment");
    
    headers.insert("X-API-KEY", HeaderValue::from_str(&serper_api_key)
            .expect("Unable to read Serper API Key into HTTP header"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Making the POST request
    let res = client.post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await;

    let res = match res {
        Ok(result) => {result},
        Err(_) => {return String::from("Google Search failed")}
    };

    // Reading the response
    let data = match res.text().await {
        Ok(data) => {data},
        Err(_) => {return String::from("Google Search failed")}
    };
    let result:SerperResults = match serde_json::from_str(&data){
        Ok(result) => {result},
        Err(_) => {return String::from("Google Search failed")}
    };

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

