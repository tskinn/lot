use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub movies: Movies,
    pub series: Series
}

pub type Movies = HashMap<String, Movie>;
pub type Series = HashMap<String, HashMap<usize, HashMap<usize, Episode>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    pub file: String,
    pub path: String,
    pub mime: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    pub file: String,
    pub mime: String,
    pub path: String,
    pub season: usize,
    pub series: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct Season {
//     pub episodes: HashMap<usize, Episode>
// }
