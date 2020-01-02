use actix_files::NamedFile;
//use actix_cors::Cors;
use actix_web::{
    client, dev::RequestHead, get, web, App, Error, HttpRequest,
    HttpServer, Responder, HttpResponse
};
use std::sync::{Mutex,RwLock,Arc};
use std::path::{PathBuf, Path};
use http::StatusCode;

mod store;
mod models;

use models::VideoStore;

async fn index() -> impl Responder {
    format!("Hello")
}

async fn index_test(req: HttpRequest, data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<NamedFile> {
    println!("{:?}", req);
    let id: String = req.match_info().query("id").parse().unwrap();
    let movie = data.movies.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;
 
    Ok(NamedFile::open(movie.path.clone())?)
}

async fn stream_movie(req: HttpRequest, data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<actix_files::NamedFile> {
    let id: String = req.match_info().query("id").parse().unwrap();
    let movie = data.movies.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;

    Ok(actix_files::NamedFile::open(movie.path.clone())?)
}

async fn list_episodes(data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<HttpResponse> {
    let episodes = data.get_episode_all().unwrap();
    Ok(HttpResponse::Ok().json(episodes))
}

async fn list_movies(data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<HttpResponse> {
    let movies = data.get_movie_all().unwrap();
    Ok(HttpResponse::Ok().json(movies))
}

async fn stream_episode(req: HttpRequest, data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<actix_files::NamedFile> {
    let id: String = req.match_info().query("id").parse().unwrap();
    let episode = data.episodes.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;

    Ok(actix_files::NamedFile::open(episode.path.clone())?)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let filename = "./hello";
    let data = match models::MemoryVideoStore::from_file(&PathBuf::from(&filename)) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            println!("Unable to load storage at: {}", filename);
            return Ok(());
        }
    };

    println!("hellooo");
    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .service(web::scope("/movies")
                     .route("/", web::get().to(list_movies))
                     .route("/{id}", web::get().to(stream_movie)))
            .service(web::scope("/episodes")
                     .route("/", web::get().to(list_episodes))
                     .route("/{id}", web::get().to(stream_episode)))
            .service(web::resource("/health").to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
