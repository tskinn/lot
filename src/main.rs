use actix_files::NamedFile;
use actix_cors::Cors;
use actix_web::{
    client, dev::RequestHead, get, web, App, Error, HttpRequest,
    HttpServer, Responder, HttpResponse
};
use std::sync::{Mutex,RwLock,Arc};
use std::path::{PathBuf, Path};
use http::StatusCode;
use clap;

mod store;
mod models;

use models::{VideoStore,Data};

async fn index() -> impl Responder {
    format!("check")
}

async fn stream_movie(req: HttpRequest, data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<actix_files::NamedFile> {
    let id: String = req.match_info().query("id").parse().unwrap();
    let movie = data.movies.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;

    Ok(actix_files::NamedFile::open(movie.path.clone())?)
}

async fn list_episodes(data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<HttpResponse> {
    let resp = Data{
        data: data.get_episode_all().unwrap()
    };
    Ok(HttpResponse::Ok().json(resp))
}

async fn list_movies(data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<HttpResponse> {
    let resp = Data{
        data: data.get_movie_all().unwrap()
    };

    Ok(HttpResponse::Ok().json(resp))
}

async fn stream_episode(req: HttpRequest, data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<actix_files::NamedFile> {
    let id: String = req.match_info().query("id").parse().unwrap();
    let episode = data.episodes.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;

    Ok(actix_files::NamedFile::open(episode.path.clone())?)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("lot").about("serve some video")
        .arg(clap::Arg::with_name("address")
             .short("a")
             .long("address")
//             .multiple(true) // TODO future
             .default_value("127.0.0.1:8080")
             .takes_value(true)
             .help("address to listen on. 127.0.0.1:8080"))
        .arg(clap::Arg::with_name("path")
             .short("p")
             .long("path")
             .required(true)
             .takes_value(true)
             .help("path to json file with video info")).get_matches();

    let address = matches.value_of("address").unwrap();
    let store_path = matches.value_of("path").unwrap();
    let data = match models::MemoryVideoStore::from_file(&PathBuf::from(&store_path)) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            println!("Unable to load storage at: {}", store_path);
            return Ok(());
        }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .send_wildcard()
                    .max_age(3600)
                    .finish())
            .data(data.clone())
            .service(web::scope("/movies")
                     .route("/", web::get().to(list_movies))
                     .route("/{id}", web::get().to(stream_movie)))
            .service(web::scope("/episodes")
                     .route("/", web::get().to(list_episodes))
                     .route("/{id}", web::get().to(stream_episode)))
            .service(web::resource("/health").to(index))
    })
        .bind(address)?
        .run()
        .await
}
