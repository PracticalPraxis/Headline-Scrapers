extern crate reqwest;
extern crate scraper;
extern crate select;

use select::document::Document;
use select::predicate::{Class};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::OpenOptions;
use std::env;
use std::process;

fn main() {
	// Get CLI Args
	let args: Vec<String> = env::args().collect();
	let file_to_write = &args[1];
	let news_source = &args[2];
	// Check for valid 3rd arg
	if news_source == "HN" || news_source == "FT" {
		assert!(get_new_headlines(file_to_write, news_source).is_ok());
	} else {
		println!("Invalid News Source Entered, Please Only Use 'HN' or 'FT' For The Third Argument");
		process::exit(1);
	}
}


#[tokio::main]
async fn get_new_headlines(file_to_write: &str, news_source: &str) -> Result<(), Box<dyn std::error::Error>> {

	// Set to HN values by default; never forget your roots
	let mut url = "https://news.ycombinator.com";
	let mut headline_value = "athing";
	let mut title_value = "storylink";

	// If FT is the source, switch our HTML values
	if news_source == "FT" {
	url = "https://www.ft.com";
	headline_value = "o-teaser__heading";
	title_value = "js-teaser-heading-link";
	}

	let mut line_count = 0;

	// Make Get request to the url
	let response = reqwest::get(url).await?;
	assert!(response.status().is_success());

	let body = response.text().await?;
	let document = Document::from_read(body.as_bytes()).unwrap();

	// Get the filepath, create it if it doesn't exist
	let path = file_to_write;
	let path_exists =  Path::new(path).exists();
	if path_exists == false {
	let _create_file = File::create(path)?;
	}
	// Open the file and set write mode to append
	let mut write_file = OpenOptions::new()
	.write(true)
	.append(true)
	.open(path)
	.unwrap();

	// For loop that extracts and compares story nodes
	for node in document.find(Class(headline_value)) {
			let mut line_present = false;
			let story = node
			    .find(Class(title_value))
			    .next()
			    .unwrap()
			    .text();

			// Get lines of existing file and check if they match the given story
			if let Ok(lines) = read_lines(path) {
			for line in lines {
				let unwrapped_line = line.unwrap();
				if story == unwrapped_line {
					line_present = true;
				}
			    }
			}

			if line_present == false {
				write!(write_file, "{}\n", story).ok();
				println!("New story detected: {}", story);
				line_count = line_count + 1
					     }
			}
	// For Loop ends here
	println!("New story count: {}", line_count);

	Ok(())
}

// Quick function to read lines of a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
