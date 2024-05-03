use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};

extern crate chrono;
use chrono::Utc;

mod utils;
mod messages;
use crate::messages::{search_serper_google, open_ai_response, return_open_ai_response};

// HTTPS requests are getting forwarded from nginx to localhost
const URL: &str = "127.0.0.1:61347";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/v1/chat/completions").route(web::post().to(answer_question)))
    })
    .bind(URL)?
    .run()
    .await
}

async fn answer_question(req: HttpRequest, mut body: web::Json<messages::OpenAIRequest>) -> impl Responder {

    // Check the API key in the HTTP header
    if !utils::authorize(req){
        return HttpResponse::Unauthorized().body("Invalid or missing API key")
    }

    // Load prompts from config
    let config = utils::Config::new(String::from("config.json"));

    let user_question = match body.messages.last() {
        Some(msg) => {msg.clone()},
        None => {return HttpResponse::BadRequest().body("Empty user request")}
    };

    // Create instructions and ask model for Google query 
    prepend_message(&mut body, &config.first_instruction);
    prepend_message(&mut body, &format!("The current time is {}",  Utc::now()));
    prepend_message(&mut body, &config.examples);

    let response = match open_ai_response(&*body).await {
        Ok(resp) => {resp},
        Err(e) => {return HttpResponse::ServiceUnavailable().body(format!("Open AI unreachable. {}", e))}
    };
    
    // Run first search query based on Models output and append results.
    append_message(&mut body, &response.choices[0].message.content);
    let query = &response.choices[0].message.content;
    println!("Query 1: {}", query);
    let search_results = search_serper_google(query).await;
    append_message(&mut body, &search_results);

    // With GPT-4, run a second google search
    if body.model == "gpt-4-turbo" {
        append_message(&mut body, &config.second_instruction);
        let response = match open_ai_response(&*body).await {
            Ok(resp) => {resp},
            Err(e) => {return HttpResponse::ServiceUnavailable().body(format!("Open AI unreachable. {}", e))}
        };
        append_message(&mut body, &response.choices[0].message.content);

        let query = &response.choices[0].message.content;
        println!("Query 2: {}", query);
        let search_results = search_serper_google(&query).await;
        append_message(&mut body, &search_results)
    }

    // Insert final instructions
    append_message(&mut body, &config.final_instruction);

    // Repeat user question
    body.messages.push(user_question);
 
    // Send modified request to OpenAI and return result to user
    return_open_ai_response(&*body).await
}

fn prepend_message(body: &mut web::Json<messages::OpenAIRequest>, message: &String){
    let message  = messages::Message::new("system", &message);
    body.messages.insert(0, message);
}

fn append_message(body: &mut web::Json<messages::OpenAIRequest>, message: &String){
    let message  = messages::Message::new("system", &message);
    body.messages.push(message);
}