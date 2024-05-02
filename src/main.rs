use actix_web::{web, App, HttpServer, Responder};
use urlencoding::encode;

mod utils;

const URL: &str = "0.0.0.0:61347";

const INITIAL_INSTRUCTIONS: &str = "You are a personal assistant to answer factual questions.\
The user will ask a question and you will provide an answer as factual correct as possible.\
To help you, the titles and snippets of a google search will be provided. \
In a first step, you will see the users question. \
Your first answer shall purely and only be the query which will be send to Google.\
For the query only use alphanumeric characters.\
Only in a later step you will need to provide the answer to the question.
Your knowledge might be missing recent news. The current date is May 2024.";

const SECOND_INSTRUCTIONS: &str = "The google research results have been appendend.\
You will have the chance to run another google search.\
Use the knowledge of the previous google search and come up with a new  query\
which you think is most helpful for answering the question.\
The query shall be substantially different from the first.
The reason is that if the results have been non helpful, consider if there was a \
false premise with the users question or the first query.
Only state the query and nothing else.";

const FINAL_INSTRUCTIONS: &str = "The last message contains the Google Search results.
Please now answer the question of the user either using the Google Search results \
or purely yourself if the google search results are not helpful. \
Be as specific as possible. \
Also, you can use all characters again.\
The user expects a answer at all times, so do not mention any problems or the google search.\
Also do not defer him to do visit further websites himself. Your task is solely to provide an answer.
The users questions will be repeated.";

async fn completions(mut body: web::Json<utils::OpenAIRequest>) -> impl Responder {

    // Create instructions and run first OpenAI request. 
    let instructions  = utils::Message::new("system", INITIAL_INSTRUCTIONS);

    body.messages.insert(0, instructions);

    let response = utils::open_ai_response(&*body).await;

    body.messages.push(response.choices[0].message.clone());
    
        
    let query = response.choices[0].message.content.clone();
    println!("Query 1: {}", query);
    let query = encode(&query).into_owned();
    let google_resp = utils::search_google(&query).await;
    body.messages.push(utils::message_from_google_search(google_resp));

    // Insert second instructions
    let instructions  = utils::Message::new("system", SECOND_INSTRUCTIONS);
    body.messages.push(instructions);
    let response = utils::open_ai_response(&*body).await;

    body.messages.push(response.choices[0].message.clone());  
    let query = response.choices[0].message.content.clone();
    println!("Query 2: {}", query);
    let query = encode(&query).into_owned();
    let google_resp = utils::search_google(&query).await;
    body.messages.push(utils::message_from_google_search(google_resp));

    // Insert final instructions
    let instructions  = utils::Message::new("system", FINAL_INSTRUCTIONS);
    body.messages.push(instructions);

    // Repeat user question
    let user_question = body.messages[1].clone();
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
