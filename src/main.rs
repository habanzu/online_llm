use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use reqwest::StatusCode;
use dotenv::dotenv;
use std::env;

const URL: &str = "0.0.0.0:61347";

#[derive(Serialize, Deserialize)]
struct ChatCompletion {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
    index: i32,
    logprobs: Option<serde_json::Value>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
struct OpenAIRequest {
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

async fn completions(body: web::Json<serde_json::Value>) -> impl Responder {
    
    // Access the environment variable
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in environment");
        
    let client = Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key) 
        .json(&*body)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/v1/chat/completions").route(web::post().to(completions)))
    })
    .bind(URL)?
    .run()
    .await
}
