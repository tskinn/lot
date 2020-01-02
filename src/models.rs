use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryVideoStore {
    pub movies: HashMap<String, Movie>,
    //    pub series: Series
    pub episodes: HashMap<String, Episode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub file: String,
    pub path: String,
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
