use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind};

// errors can be created from strings
//let custom_error = Error::new(ErrorKind::Other, "oh no!");


use crate::models::{Store,Episode};

impl Store {
    pub fn from_file(path: PathBuf) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let store = serde_json::from_str(data.as_str()).map_err(|e| e.into());
        store
    }

    pub fn to_file(self, path: PathBuf) -> std::io::Result<usize> {
        let str_result = serde_json::to_string(&self)?;
        let mut file = File::open(path)?;
        file.write(str_result.as_bytes())
    }

    pub fn add_movie(&self, path: PathBuf) -> std::io::Result<()> {
        Ok(())
    }

    pub fn add_episode(&self, path: PathBuf, series: String, season: usize) -> std::io::Result<()> {
        let episode = Episode{
            series: series,
            season: season,
            file: path.to_str().unwrap_or("").to_string(),
            path: path.to_str().unwrap_or("").to_string(),
            mime: get_mime(path.extension().unwrap()),
        };

  
        let filename = path.file_stem().ok_or(Error::new(ErrorKind::Other, "failed to get stem"))?;
        let filename = filename.to_str().ok_or(Error::new(ErrorKind::Other, "failed to change ostr to str"))?;
        let filename_usize = filename.parse::<usize>().map_err(|e| e.into())?;

        match self.series.get(&series) {
            Some(seasons) => {
                // TODO check if season exists maybe
                // TODO insert episode into season
                let mut season_real = seasons.get(&season).ok_or(Error::new(ErrorKind::Other, "aanother crappy error failed to get season"))?;
                season_real.insert(1, episode);
            },
            None => {}
        }
        Ok(())
    }    
}

pub fn get_mime(extension: &OsStr) -> String {
    match extension.to_str().unwrap_or("") {
        "mp4" =>  "video/mp4".to_string(),
        _ => "video/webm".to_string()
    }
}
