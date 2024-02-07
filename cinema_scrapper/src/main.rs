extern crate reqwest;
extern crate scraper;
use std::io;
use std::fs::File;
use cinema_scrapper::scrape_time_and_title_data;

fn main(){
    
    let req = reqwest::get("https://www.cineode.fr/le-vigan-le-palace/horaires/").unwrap().text().unwrap();
    scrape_time_and_title_data(req);

    let mut resp = reqwest::get("https://www.cineode.fr/le-vigan-le-palace/horaires/").expect("request failed");
    let mut out = File::create("html_file.html").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}