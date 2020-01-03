use structopt::StructOpt;
use std::path::PathBuf;
use std::collections::HashMap;

use serde_json;

#[macro_use] extern crate failure_derive;
#[macro_use] extern crate failure;

mod models;
mod store;

use models::{MemoryVideoStore, VideoStore, Data};

#[derive(StructOpt, Debug)]
#[structopt(name = "util")]
struct Util {
    #[structopt(short, long, parse(from_os_str))]    
    json_path: PathBuf,
    #[structopt(subcommand)]
    command: SubCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    #[structopt(name = "movie")]
    Movie(Movie),

    #[structopt(name = "episode")]
    Episode(Episode)
}

#[derive(StructOpt, Debug)]
struct Movie {
    #[structopt(subcommand)]    
    action: MovieAction
}

#[derive(StructOpt, Debug)]
enum MovieAction {
    Add {
        #[structopt(parse(from_os_str))]
        movie_paths: Vec<PathBuf>
    },
    List 
}

#[derive(StructOpt, Debug)]
struct Episode {
    #[structopt(long)]
    series: String,
    #[structopt(long)]
    season: usize,
    #[structopt(subcommand)]    
    action: EpisodeAction
}

#[derive(StructOpt, Debug)]
enum EpisodeAction {
    Add {
        #[structopt(parse(from_os_str))]
        episode_paths: Vec<PathBuf>
    },
    List
}

fn main() {
    let args = Util::from_args();
    let json_path = args.json_path.clone();
    //println!("{:#?}", args);
    let mut store = match MemoryVideoStore::from_file(&json_path) {
        Ok(store) => store,
        Err(_) => {
            MemoryVideoStore::new()
        }
    };
    
    match args.command {
        SubCommand::Movie(movie) => {            
            match movie.action {
                MovieAction::Add{movie_paths} => {
                    for movie_path in movie_paths {
                        match store::add_movie(movie_path.to_path_buf(), &mut store) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{}", e);
                                println!("failed to add movie at {}", movie_path.to_str().unwrap_or("failed to unwrap movie_path"));
                            }
                        };
                        
                    }
                    store.to_file(&json_path).unwrap();
                },
                MovieAction::List => {
                    let movies = store.movies;
                    let data : Data<HashMap<String, models::Movie>> = Data{
                        data: movies,
                    };
                    let res = serde_json::to_string(&data).unwrap();
                    println!("{}", res);
                }
            }
        },
        SubCommand::Episode(episode) => {
            match episode.action {
                EpisodeAction::Add{episode_paths} => {
                    for episode_path in episode_paths {
                        match store::add_episode(episode_path.to_path_buf(), &episode.series, episode.season, &mut store) {
                            Ok(_) => {},
                            Err(err) => {
                                println!("{}", err);
                                println!("failed to add episode at {}", episode_path.to_str().unwrap_or("failed to unwrap episode_path"));
                            }
                        }

                    }
                    store.to_file(&json_path).unwrap();
                },
                EpisodeAction::List => {
                    let episodes = store.episodes;
                    let data : Data<HashMap<String, models::Episode>> = Data{
                        data: episodes,
                    };
                    let res = serde_json::to_string(&data).unwrap();
                    println!("{}", res);
                }
            }
        }
    }
}
