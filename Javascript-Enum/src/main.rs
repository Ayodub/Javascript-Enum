extern crate reqwest;
extern crate select;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the text file of URLs
    let file = File::open("urls.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Iterate through each line (URL) in the file
    for line in reader.lines() {
        let url = line.expect("Failed to read line");

        // Send a GET request to the URL and parse the HTML
        let body = reqwest::get(&url).expect("Failed to send request").text().expect("Failed to get response text");
        let document = select::document::Document::from(body.as_str());

        // Search for the terms in the JavaScript
        for script in document.find(select::predicate::Name("script")) {
            let script_text = script.text();
            if script_text.contains("secret") || script_text.contains("key") || script_text.contains("password") {
                println!("Found term in script on {}", url);
            }
        }
    }
}
