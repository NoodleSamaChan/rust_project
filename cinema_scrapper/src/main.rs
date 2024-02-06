extern crate reqwest;
extern crate scraper;

use scraper::{ElementRef, Html, Selector};

fn main(){
    scrape_team_data("https://www.cineode.fr/le-vigan-le-palace/horaires/semaine-prochaine/");
}

fn scrape_team_data(url:&str){

    let mut req = reqwest::get(url).unwrap();
    assert!(req.status().is_success());
    let doc_body = Html::parse_document(&req.text().unwrap());

    let block_selector = Selector::parse(".wrap-fiche-film").unwrap();
    //let select_block = doc_body.select(&block_selector).next().unwrap();

    let titre = Selector::parse("div > div > h4 > a").unwrap(); 

    for element in doc_body.select(&block_selector){

        for titre_movie in element.select(&titre){
            let titre_movie = titre_movie.text().collect::<Vec<_>>();
            println!("{:?}", titre_movie);
        }

        let time = Selector::parse(".hor").unwrap();

        for time in element.select(&time){
            let times = time.text().collect::<Vec<_>>();
            println!("{:?}", times);
        }
    }
}