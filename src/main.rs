use serde::{Deserialize, Serialize};
use reqwest::StatusCode;
use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use log::info;

mod utils;

const URL: &str = "0.0.0.0:61347";
const INSTRUCTIONS: &str = "You are a search engine and personal assistant. A list of the titles and and snippets of the first 10 google results will be provided additionally to the question. Answer the question to the best of your possibility given the limited information. Include links to sources if possible not counting against the word limit.";

async fn completions(mut body: web::Json<utils::OpenAIRequest>) -> impl Responder {

    // Create instructions and prepend to request. 
    let instructions  = utils::Message::new("system", INSTRUCTIONS);

    body.messages.insert(0, instructions);

    //Extract user question and send to Google
    let last_message = match body.messages.last() {
        Some(message) => {Ok(message.content.clone())},
        None => {Err("Received message was empty.")},
    }.expect("Received message was empty.");

    let query = &last_message;

    let google_resp = utils::search_google(query).await;

    body.messages.push(utils::message_from_google_search(google_resp));
 
    // Send modified request to OpenAI and return result
    utils::return_open_ai_response(&*body).await
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
