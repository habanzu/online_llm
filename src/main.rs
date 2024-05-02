use actix_web::{web, App, HttpServer, Responder};
use urlencoding::encode;
use std::fs;
use serde::Deserialize;

use crate::utils::Message;
extern crate chrono;
use chrono::{DateTime, Utc};

mod utils;

const URL: &str = "0.0.0.0:61347";

#[derive(Deserialize)]
struct Config {
    examples: String,
    first_instruction: String,
    second_instruction: String,
    final_instruction: String
}

async fn completions(mut body: web::Json<utils::OpenAIRequest>) -> impl Responder {
    // Load prompts from config
    let file_path = "src/config.json";
    let file_content = fs::read_to_string(file_path).expect("Could not read file.");
    let config: Config = serde_json::from_str(&file_content).expect("Could not parse config");

    let user_question = body.messages[0].clone();

    //Decide whether to ask ChatGPT a second time for a query. Only worthwhile for GPT 4.
    let high_accuracy = body.model == "gpt-4-turbo";

    // Create instructions and run first OpenAI request. 
    let examples  = utils::Message::new("system", &config.examples);
    let instructions  = utils::Message::new("system", &config.first_instruction);
    let utc_now: DateTime<Utc> = Utc::now();
    let time_message = utils::Message::new(String::from("system"), format!("The current time is {}", utc_now));
    println!("{}", time_message.content);

    body.messages.insert(0, time_message);
    body.messages.insert(0, instructions);
    body.messages.insert(0, examples);

    body.model = String::from("gpt-3.5-turbo-16k");
    let response = utils::open_ai_response(&*body).await;
    if high_accuracy {
        body.model = String::from("gpt-4-turbo");
    }
    
    // Run first search query based on Models output.
    body.messages.push(response.choices[0].message.clone());
        
    let query = response.choices[0].message.content.clone();
    println!("Query 1: {}", query);
    // let query = encode(&query).into_owned();
    let data = utils::search_serper_google( &query).await;
    // println!("{}",data);
    let results_message = Message::new("system", &data);
    body.messages.push(results_message);
    // let google_resp = utils::search_google(&query).await;
    // body.messages.push(utils::message_from_google_search(google_resp));

    // Insert second instructions
    if high_accuracy {
        let instructions  = utils::Message::new("system", &config.second_instruction);
        body.messages.push(instructions);
        let response = utils::open_ai_response(&*body).await;

        body.messages.push(response.choices[0].message.clone());  
        let query = response.choices[0].message.content.clone();
        println!("Query 2: {}", query);
        let query = encode(&query).into_owned();
        let data = utils::search_serper_google( &query).await;
        let results_message = Message::new("system", &data);
        body.messages.push(results_message);
        println!("{}", data);
    }

    // Insert final instructions
    let instructions  = utils::Message::new("system", &config.final_instruction);
    body.messages.push(instructions);

    // Repeat user question
    body.messages.push(user_question);
 
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
