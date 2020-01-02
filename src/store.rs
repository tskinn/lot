use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind};
use nanoid;

// errors can be created from strings
//let custom_error = Error::new(ErrorKind::Other, "oh no!");

use crate::models::{MemoryVideoStore,Episode,Movie,VideoStore};

impl VideoStore for MemoryVideoStore {
    fn add_episode(&mut self, episode: Episode) -> Result<(), String> {
        self.episodes.insert(episode.id.clone(), episode);
        Ok(())
    }

    fn delete_episode(&mut self, id: &str) -> Result<(), String> {
        self.episodes.remove(id);
        Ok(())
    }
    
    fn get_episode_by_id(&self, id: &str) -> Result<&Episode, String> {
        self.episodes.get(id).ok_or("crap".to_string())
    }

    fn get_episode_all(&self) -> Result<Vec<&Episode>, String> {
        let episodes: Vec<&Episode> = self.episodes.iter().map(|x| x.1).collect();
        Ok(episodes)
    }    

    fn add_movie(&mut self, movie: Movie) -> Result<(), String> {
        self.movies.insert(movie.id.clone(), movie);
        Ok(())
    }
    
    fn delete_movie(&mut self, id: &str) -> Result<(), String> {
        self.movies.remove(id);
        Ok(())
    }
    
    fn get_movie_by_id(&self, id: &str) -> Result<&Movie, String> {
        self.movies.get(id).ok_or(format!("couldn't find movie with id of {}", id))
    }

    fn get_movie_all(&self) -> Result<Vec<&Movie>, String> {
        let movies: Vec<&Movie> = self.movies.iter().map(|x| x.1).collect();
        Ok(movies)
    }    
}

impl MemoryVideoStore {
    pub fn new() -> Self {
        MemoryVideoStore{
            movies: HashMap::new(),
            episodes: HashMap::new()
        }
    }
    
    pub fn from_file(path: &PathBuf) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let store = serde_json::from_str(data.as_str()).map_err(|e| e.into());
        store
    }

    pub fn to_file(&self, path: &PathBuf) -> std::io::Result<usize> {
        let str_result = serde_json::to_string(&self)?;
        let mut file = File::create(path)?;
        file.write(str_result.as_bytes())
    }
}

pub fn get_mime(extension: &OsStr) -> String {
    match extension.to_str().unwrap_or("") {
        "mp4" =>  "video/mp4".to_string(),
        _ => "video/webm".to_string()
    }
}



pub fn add_episode(path: PathBuf, series: String, season: usize, store: &mut impl VideoStore) -> Result<(), String> {
    let filename = path.file_stem().ok_or("failed to get file stem".to_string())?;
    let filename = filename.to_str().ok_or("failed to get filename as string".to_string())?;
    let filename_usize = filename.parse::<usize>().unwrap();

    let episode = Episode{
        series: series.clone(),
        season: season,
        file: path.to_str().unwrap_or("").to_string(),
        path: path.to_str().unwrap_or("").to_string(),
        mime: get_mime(path.extension().unwrap()),
        id: nanoid::simple(),
        number: filename_usize,
    };

    store.add_episode(episode)?;
    Ok(())
}

pub fn add_movie(path: PathBuf, store: &mut impl VideoStore) -> Result<(), String> {
    let id = nanoid::simple();
    let mime = get_mime(path.extension().unwrap_or(OsStr::new("")));
    let movie_name = path.file_stem().ok_or("failed to get file stem".to_string())?.to_str().ok_or("failed to get stem as str")?;

    let movie = Movie{
        file: path.to_str().unwrap_or("").to_string(),
        path: path.to_str().unwrap_or("").to_string(),
        mime: mime,
        title: movie_name.to_string(),
        id: id.clone(),
    };

    store.add_movie(movie)
}
