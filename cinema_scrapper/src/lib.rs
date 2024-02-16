extern crate reqwest;
extern crate scraper;
use scraper::{Html, Selector};
use time::Time;
use time::macros::format_description;
use time::OffsetDateTime;

#[derive(Debug)]
#[derive(Clone)]

//movie title and time structure
pub struct MovieTimes {
    pub title: String,
    pub times: Vec<time::Time>,
}

pub fn build_scheduler (title_to_add: String, times_to_add: Vec<time::Time>) -> MovieTimes {
    if times_to_add.is_empty() == false {
        MovieTimes{
            title: title_to_add,
            times: times_to_add,
        }
    } else {
        MovieTimes{
            title: String::new(),
            times: Vec::new(),
        }
    }
}

impl MovieTimes {
    pub fn time_left_till_next_viewing (&self, t_time: u8) -> Option<u8> {
        self.times.iter().filter(|element| t_time < element.hour()).map(|element|element.hour() - t_time).min()
    }
    
}

pub fn scrape_time_and_title_data(html_file:String) -> Vec<MovieTimes>{

    let doc_body = Html::parse_document(&html_file);
    
    //selectors of htlm code
    let block_selector = Selector::parse(".wrap-fiche-film").unwrap();
    let titre = Selector::parse("div > div > h4 > a").unwrap(); 
    let day = Selector::parse(".today").unwrap();

    let mut titles_of_movies: String = String::new();
    let mut times_of_movies:Vec<time::Time> = Vec::new();
    let mut vec_strcut_movies: Vec<MovieTimes> = Vec::new();
    let format = format_description!("[hour]h[minute]");

    //loops to gather information based on the selectors
    for element in doc_body.select(&block_selector){

        //title selector
        for titre_movie in element.select(&titre){
            let titre_movie: String = titre_movie.text().map(|element|element.to_string()).next().unwrap();
            titles_of_movies = titre_movie;
        }
        
        //day "today" selector
        for day in element.select(&day){

            //time selector
            let time = Selector::parse(".hor").unwrap();

            for time in day.select(&time){
                let times: Vec<time::Time> = time.text().clone().map(|element|(Time::parse(element, &format).expect(element))).collect::<Vec<_>>();
                times_of_movies.extend(times.clone());
            }
        }
        
        //time duplication delation
        times_of_movies.dedup();

        //structure creation
        if times_of_movies.is_empty() != true {
            let movie_sched = build_scheduler(titles_of_movies.clone(), times_of_movies.clone());
            vec_strcut_movies.push(movie_sched);
        }

        //reseting of variables
        titles_of_movies = String::new();
        times_of_movies = Vec::new();
    }

    return vec_strcut_movies
}

pub fn next_movie(schedule:Vec<MovieTimes>) -> MovieTimes {
    let t_time = OffsetDateTime::now_utc().hour();

    schedule.iter().min_by_key(|element|element.time_left_till_next_viewing(t_time)).unwrap().clone()

}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_scrape_from_html() {
        let file_to_convert = include_str!("../tests/html_file.html").to_string();

        let vec_to_compare: Vec<String> = vec!["\tMaison de retraite 2\t", "16h00", "\tCocorico\t", "20h30", "16h00", "18h00", "20h30", "18h00", "16h00", "20h30", "\tLe Dernier Jaguar\t", "18h00", "16h00", "14h00", "14h00", "14h00", "14h00", "\tLéo, la fabuleuse histoire de Léonard de Vinci\t", "14h00", "11h00", "\tLe Dernier des Juifs\t", "16h00", "20h00", "\tVivre avec les loups\t", "16h00", "18h00", "16h00", "\tPauvres Créatures\t", "18h00", "20h00", "18h00", "18h00", "20h00", "\tPierre Feuille Pistolet\t", "20h00"].iter().map(|element|element.to_string()).collect();
        
        let result  = scrape_time_and_title_data(file_to_convert);
        assert_eq!(result, vec_to_compare);
    }
}