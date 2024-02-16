extern crate reqwest;
extern crate scraper;
use std::io;
use std::fs::File;
use cinema_scrapper::scrape_time_and_title_data;
use cinema_scrapper::next_movie;

fn main(){
    
    let req = reqwest::get("https://www.cineode.fr/le-vigan-le-palace/horaires/").unwrap().text().unwrap();
    let struct_creator = scrape_time_and_title_data(req);
    let next_movie_viewing = next_movie(struct_creator);

    println!("{:#?}", next_movie_viewing);

//    Notification::new()
//        .summary("Oh by the way")
//        .body(&format!("you can catch {:?} at {:?}", duration))
//        .schedule(chrono::Utc::now() + duration)?;
}