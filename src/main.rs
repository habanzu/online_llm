use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use urlencoding::encode;

use crate::utils::Message;
extern crate chrono;
use chrono::{DateTime, Utc};

mod utils;

// HTTPS requests are getting forwarded from nginx to localhost
const URL: &str = "127.0.0.1:61347";

fn prepend_message(body: &mut web::Json<utils::OpenAIRequest>, message: &String){
    let message  = utils::Message::new("system", &message);
    body.messages.insert(0, message);
}

fn append_message(body: &mut web::Json<utils::OpenAIRequest>, message: &String){
    let message  = utils::Message::new("system", &message);
    body.messages.push(message);
}

async fn completions(req: HttpRequest, mut body: web::Json<utils::OpenAIRequest>) -> impl Responder {
    // Check the API key in the HTTP header
    if !utils::authorize(req){
        return HttpResponse::Unauthorized().body("Invalid or missing API key")
    }

    // Load prompts from config
    let config = utils::Config::new(String::from("config.json"));

    let user_question = body.messages[0].clone();

    let pose_second_query = body.model == "gpt-4-turbo";

    // Create instructions and run first OpenAI request. 
    prepend_message(&mut body, &config.first_instruction);
    prepend_message(&mut body, &format!("The current time is {}",  Utc::now()));
    prepend_message(&mut body, &config.examples);

    let response = utils::open_ai_response(&*body).await;
    
    // Run first search query based on Models output and append results.
    append_message(&mut body, &response.choices[0].message.content);
    let query = &response.choices[0].message.content;
    println!("Query 1: {}", query);
    let search_results = utils::search_serper_google(query).await;
    append_message(&mut body, &search_results);

    // With GPT-4, run a second google search
    if pose_second_query {
        append_message(&mut body, &config.second_instruction);
        let response = utils::open_ai_response(&*body).await;
        append_message(&mut body, &response.choices[0].message.content);

        let query = &response.choices[0].message.content;
        println!("Query 2: {}", query);
        let search_results = utils::search_serper_google(&query).await;
        append_message(&mut body, &search_results)
    }

    // Insert final instructions
    append_message(&mut body, &config.final_instruction);

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
