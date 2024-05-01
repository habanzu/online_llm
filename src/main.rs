use serde::Deserialize;
use reqwest::{Error, StatusCode};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let google_api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY not found in environment");
    let cx = env::var("SEARCH_ENGINE_ID").expect("SEARCH_ENGINE_ID not found in environment");
        
    let query = "Robert Habeck";

    let url = format!(
        "https://www.googleapis.com/customsearch/v1?q={}&key={}&cx={}",
        query, google_api_key, cx
    );

    match reqwest::get(&url).await {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => match response.json::<GoogleSearchResults>().await {
                    Ok(google_resp) => {
                        for item in google_resp.items {
                            println!("Title: {}, Snippet: {}", item.title, item.snippet);
                        }
                    },
                    Err(e) => {
                        println!("Error when deserializing Google response {:?}", e);
                    }
                },
                _ => {
                    println!("HTTP Request with bad status code.")
                }
            }
        },
        Err(e) => {
            println!("Failed to send request to Google: {:?}", e);
        },
    }; 

    println!("Printing items.");

    Ok(())
}

#[derive(Deserialize)]
struct GoogleSearchResults {
    kind: String,
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    title: String,
    link: String,
    snippet: String,
}
