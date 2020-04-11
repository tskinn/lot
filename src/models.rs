use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonVideoStore {
    pub movies: HashMap<String, Movie>,
    pub episodes: HashMap<String, Episode>,
}

// pub struct VideoService {
//     pub movies_by_id: HashMap<String, Arc<Movie>>,
//     pub movies_by_title: Vec<(String, String)>,
//     pub movies_id_by_genre: HashMap<usize, Vec<String>>,

//     pub episodes_by_id: HashMap<String, Episode>,
//     pub episodes_by_season: HashMap<String, HashMap<usize, Vec<String>>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub file: String,
    pub path: String,
    pub mime: String,
    pub title: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMovie {
    pub mime: String,
    pub title: String,
    pub id: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Episode {
    pub file: String,
    pub mime: String,
    pub path: String,
    pub season: usize,
    pub series: String,
    pub id: String,
    pub number: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseEpisode {
    pub mime: String,
    pub season: usize,
    pub series: String,
    pub id: String,
    pub number: usize,
}

pub trait VideoStore {
    fn add_episode(&mut self, episode: Episode) -> Result<(), String>;
    fn delete_episode(&mut self, id: &str) -> Result<(), String>;
    fn get_episode_by_id(&self, id: &str) -> Result<&Episode, String>;
    fn get_episode_all(&self) -> Result<Vec<&Episode>, String>;
    
    fn add_movie(&mut self, movie: Movie) -> Result<(), String>;
    fn delete_movie(&mut self, id: &str) -> Result<(), String>;
    fn get_movie_by_id(&self, id: &str) -> Result<&Movie, String>;
    fn get_movie_all(&self) -> Result<Vec<&Movie>, String>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data<T> {
    pub data: T
}

