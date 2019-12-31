use structopt::StructOpt;
use std::path::PathBuf;

#[macro_use] extern crate failure_derive;
#[macro_use] extern crate failure;

mod models;
mod store;

use models::Store;

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
    println!("{:#?}", args);
    match args.command {
        SubCommand::Movie(movie) => {
            match movie.action {
                MovieAction::Add{movie_paths} => {
                    println!("{:?}", movie_paths);
                },
                MovieAction::List => {
                    let store = Store::from_file(args.json_path).unwrap();
                    println!("{:#?}", store);
                }
            }
        },
        SubCommand::Episode(episode) => {
            match episode.action {
                EpisodeAction::Add{episode_paths} => {
                    println!("{:?}", episode_paths);
                    let mut store = Store::from_file(args.json_path).unwrap();

                    match store.add_episode(episode_paths.first().unwrap().to_path_buf(), episode.series, episode.season) {
                        Ok(v) => {
                            println!("{:?}", v);
                            println!("how did this wokr");
                        },
                        Err(err) => {
                            println!("wtf");
                            println!("{}", err);
                        }
                    }
                    println!("{:#?}", store);
                },
                EpisodeAction::List => {
                    println!("hello");
                }
            }
        }
    }
}
