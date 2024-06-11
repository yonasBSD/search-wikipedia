use serde::{Serialize};
use serde_json::json;
use reqwest::blocking::Client;
use clap::{App, Arg};
use colored::*;
use colored_json::prelude::*;

#[derive(Serialize)]
struct SearchResult {
    title: String,
    url: String,
}

fn search_wikipedia(search_term: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get("https://en.wikipedia.org/w/api.php")
        .query(&[
            ("action", "query"),
            ("format", "json"),
            ("list", "search"),
            ("srsearch", search_term),
        ])
        .send()?
        .text()?;
    
    Ok(response)
}

fn main() {
    let args = App::new("Wikipedia Search")
        .version("1.0")
        .author("Your Name")
        .about("Search Wikipedia and display results")
        .arg(Arg::with_name("query")
            .help("Sets the search query")
            .required(true)
            .index(1))
        .arg(Arg::with_name("json")
            .short('j')
            .long("json")
            .help("Output search results in JSON format"))
        .get_matches();

    let search_term = args.value_of("query").unwrap();

    match search_wikipedia(search_term) {
        Ok(response) => {
            let parsed_response: serde_json::Value = serde_json::from_str(&response).expect("Failed to parse JSON");
            if let Some(results) = parsed_response["query"]["search"].as_array() {
                if args.is_present("json") {
                    let search_results: Vec<SearchResult> = results.iter().map(|result| {
                        let title = result["title"].as_str().unwrap().to_string();
                        let url = format!("https://en.wikipedia.org/wiki/{}", title.replace(" ", "_"));
                        SearchResult { title, url }
                    }).collect();
                    let json_output = json!(search_results);
                    println!("{}", serde_json::to_string_pretty(&json_output).unwrap().to_colored_json_auto().unwrap());
                } else {
                    println!("{}:", format!("Search results for '{}'", search_term).yellow().bold());

                    for result in results {
                        let title = result["title"].as_str().unwrap();
                        let url = format!("https://en.wikipedia.org/wiki/{}", title.replace(" ", "_"));
                        println!(" - {}: {}", title.green(), url.blue()); // Colorize title and URL
                    }
                }
            } else {
                println!("No search results found for '{}'", search_term);
            }
        }
        Err(e) => {
            eprintln!("Error searching Wikipedia: {}", e);
        }
    }
}
