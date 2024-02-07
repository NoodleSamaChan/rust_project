extern crate reqwest;
extern crate scraper;
use scraper::{Html, Selector};

pub fn scrape_time_and_title_data(html_file:String) -> Vec<String>{

    let doc_body = Html::parse_document(&html_file);

    let block_selector = Selector::parse(".wrap-fiche-film").unwrap();

    let titre = Selector::parse("div > div > h4 > a").unwrap(); 

    let mut collection_of_times_titles:Vec<&str>= Vec::new();

    for element in doc_body.select(&block_selector){

        for titre_movie in element.select(&titre){
            let titre_movie = titre_movie.text().collect::<Vec<_>>();
            collection_of_times_titles.extend(&titre_movie);
            println!("{:?}", titre_movie);
            
        }

        let time = Selector::parse(".hor").unwrap();

        for time in element.select(&time){
            let times = time.text().collect::<Vec<_>>();
            collection_of_times_titles.extend(&times);
            println!("{:?}", times);
        }
    }

    let mut final_vector: Vec<String> = Vec::new();

    for value in collection_of_times_titles{
        final_vector.push(value.to_string())

    }

    return final_vector
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