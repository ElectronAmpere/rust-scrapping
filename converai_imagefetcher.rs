use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;
use std::path::Path;
use reqwest::blocking::Client;
use url::Url;

// Gets the file name from the url
fn get_filename_from_url(url: &str) -> String {
    let parsed_url = Url::parse(url).unwrap();
    Path::new(&parsed_url.path())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

// Will download the images from the links present in the file
fn download_images_from_file(input_file: &str) {
    let client = Client::new();
    for line in BufReader::new(File::open(input_file).unwrap()).lines() {
        let url = line.unwrap();
        if url.contains("/disp/") {
            let image_url = url.replace("/disp/", "/source/");
            let filename = get_filename_from_url(&image_url);
            let response = client.get(&image_url).send().unwrap();
            let mut file = File::create(filename).unwrap();
            file.write_all(&response.bytes().unwrap()).unwrap();
        } else if url.contains("/fs/") {
            let image_url = url.replace("/fs/", "/source/");
            let filename = get_filename_from_url(&image_url);
            let response = client.get(&image_url).send().unwrap();
            let mut file = File::create(filename).unwrap();
            file.write_all(&response.bytes().unwrap()).unwrap();
        } else {
            println!("Link error");
        }
    }
}

// Remove all duplicate lines from a file, File must end with a new line
fn remove_duplicates_from_file(input_file: &str, output_file: &str) {
    let mut lines_seen = HashSet::new();
    let mut output_file = File::create(output_file).unwrap();
    for line in BufReader::new(File::open(input_file).unwrap()).lines() {
        let line = line.unwrap();
        if !lines_seen.contains(&line) {
            output_file.write_all(line.as_bytes()).unwrap();
            output_file.write_all(b"\n").unwrap();
            lines_seen.insert(line);
        }
    }
}

// Fetch website data from a file
fn fetch_website_url_from_file(input_file: &str, output_file: &str) {
    let mut output_file = File::create(output_file).unwrap();
    let client = Client::new();
    for url in BufReader::new(File::open(input_file).unwrap()).lines() {
        let url = url.unwrap().trim().to_string();
        let response = client.get(&url).send().unwrap();
        if response.status().is_success() {
            let html_file = response.text().unwrap();
            let soup = scraper::Html::parse_document(&html_file);
            for img_tag in soup.select("img") {
                if let Some(link) = img_tag.value().attr("srcset") {
                    let link = link.split(" ").next().unwrap();
                    output_file.write_all(link.as_bytes()).unwrap();
                    output_file.write_all(b"\n").unwrap();
                }
            }
        }
        println!("Completed = {}", url);
    }
}

fn main() {
    let input_file = "imageurls.txt";
    let output_file = format!("{}_parsed.txt", input_file.split('.').next().unwrap());
    let output_file_1 = format!("{}_parsed.txt", output_file.split('.').next().unwrap());

    remove_duplicates_from_file(input_file, &output_file);
    download_images_from_file(&output_file);
}

