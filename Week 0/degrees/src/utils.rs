use std::collections::{HashMap, HashSet};
// use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::graph::{People, Movie, Graph};

pub fn load_data(directory: &str) -> Graph {
    let mut people: HashMap<u32, People> = HashMap::new();
    let mut movies: HashMap<u32, Movie> = HashMap::new();

    // Load people.csv which contains id, name, birth
    let people_file: File = File::open(format!("{}/people.csv", directory)).unwrap();
    let reader: BufReader<File> = BufReader::new(people_file);

    let lines: std::iter::Skip<std::io::Lines<BufReader<File>>> = reader.lines().skip(1);

    for line in lines {

        let line = line.unwrap();
        let mut record = line.split(',');
        let id = record.next().unwrap().parse::<u32>().unwrap();
        let mut name = record.next().unwrap().to_string();
        if name.ends_with("\"") {
            name.remove(0);
            name.pop();
        }
        let birth = record.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
        let movies = HashSet::new();
        people.insert(id, People { name, birth, movies });
    }

    // Load movies.csv which contains id, title, year 
    let movies_file = File::open(format!("{}/movies.csv", directory)).unwrap();
    let reader = BufReader::new(movies_file);

    let lines = reader.lines().skip(1);

    for line in lines {
        let line = line.unwrap();
        let mut record = line.split(',');
        let id = record.next().unwrap().parse::<u32>().unwrap();
        let mut title = record.next().unwrap().to_string();
        if title.ends_with("\"") {
            title.remove(0);
            title.pop();
        }
        let year = record.next().unwrap_or("00").parse::<u16>().unwrap_or(0);
        let stars = HashSet::new();

        movies.insert(id, Movie { title, year, stars });
    }

    // Load stars.csv which contains person_id, movie_id,
    let stars_file = File::open(format!("{}/stars.csv", directory)).unwrap();
    let reader = BufReader::new(stars_file);

    let lines = reader.lines().skip(1);


    for line in lines {
        let line = line.unwrap();
        let mut record = line.split(',');

        let person_id = record.next().unwrap().parse::<u32>().unwrap();
        let movie_id = record.next().unwrap().parse::<u32>().unwrap();

        if let Some(person) = people.get_mut(&person_id) {
            person.movies.insert(movie_id);
        }

        if let Some(movie) = movies.get_mut(&movie_id) {
            movie.stars.insert(person_id);
        }
    }
    Graph::new(people, movies)
}

// pub fn read_input() -> String {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     input.trim().to_string()
// }