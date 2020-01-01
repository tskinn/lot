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


use crate::models::{Store,Episode,Movie};

impl Store {
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

    pub fn add_episode(&mut self, path: PathBuf, series: String, season: usize) -> std::io::Result<()> {
        let episode = Episode{
            series: series.clone(),
            season: season,
            file: path.to_str().unwrap_or("").to_string(),
            path: path.to_str().unwrap_or("").to_string(),
            mime: get_mime(path.extension().unwrap()),
            id: nanoid::simple(),
        };

  
        let filename = path.file_stem().ok_or(Error::new(ErrorKind::Other, "failed to get stem"))?;
        let filename = filename.to_str().ok_or(Error::new(ErrorKind::Other, "failed to change ostr to str"))?;
        let filename_usize = filename.parse::<usize>().unwrap();

        let seasons = self.series.entry(series).or_insert(HashMap::new());
        let season = seasons.entry(season).or_insert(HashMap::new());
        season.insert(filename_usize, episode);

        Ok(())
    }

    pub fn add_movie(&mut self, path: PathBuf) -> std::io::Result<()> {
        let id = nanoid::simple();
        let mime = get_mime(path.extension().unwrap_or(OsStr::new("")));
        let movie_name = path.file_stem().ok_or(Error::new(ErrorKind::Other, "no filename!"))?.to_str().ok_or(Error::new(ErrorKind::Other, "crap me  river"))?;

        let movie = Movie{
            file: path.to_str().unwrap_or("").to_string(),
            path: path.to_str().unwrap_or("").to_string(),
            mime: mime,
            title: movie_name.to_string(),
            id: id.clone(),
        };

        self.movies.insert(id, movie);
        Ok(())
    }
}

pub fn get_mime(extension: &OsStr) -> String {
    match extension.to_str().unwrap_or("") {
        "mp4" =>  "video/mp4".to_string(),
        _ => "video/webm".to_string()
    }
}
