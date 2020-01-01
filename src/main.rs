use actix_files;
//use actix_cors::Cors;
use actix_web::{
    client, dev::RequestHead, get, web, App, Error, HttpRequest,
    HttpServer, Responder,
};
use std::sync::{Mutex,RwLock,Arc};
use std::path::{PathBuf, Path};
use http::StatusCode;

mod store;
mod models;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(usize, String)>, data: web::Data<models::MemoryVideoStore>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}


fn stream_movie(req: HttpRequest, data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<actix_files::NamedFile> {
    let id: String = req.match_info().query("id").parse().unwrap();
    let movie = data.movies.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;
    let path = movie.path;
    Ok(actix_files::NamedFile::open(path)?)
}

fn stream_episode(req: HttpRequest, data: web::Data<models::MemoryVideoStore>) -> actix_web::Result<actix_files::NamedFile> {
    let id: String = req.match_info().query("id").parse().unwrap();
    let movie = data.episodes.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;
    let path = movie.path;
    Ok(actix_files::NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = models::MemoryVideoStore::from_file(&PathBuf::from("./hello"));
    
    HttpServer::new(|| {
        App::new()
            .app_data(data)
            .service(index)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
