extern crate reqwest;
extern crate scraper;
use cinema_scrapper::scrape_time_and_title_data;
use cinema_scrapper::next_movie;
use notify_rust::Notification;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let req = reqwest::get("https://www.cineode.fr/le-vigan-le-palace/horaires/").unwrap().text().unwrap();
    let struct_creator = scrape_time_and_title_data(req);
    let next_movie_viewing = next_movie(struct_creator);

    println!("{:#?}", next_movie_viewing);

    Notification::new()
        .summary("Oh by the way")
        .body(&format!("you can catch {:?} at {:?}", next_movie_viewing.title, next_movie_viewing.times))
        .show()?;
    Ok(())

}