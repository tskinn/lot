use actix_files;
//use actix_cors::Cors;
use actix_web::{
    client, dev::RequestHead, get, web, App, Error, HttpRequest,
    HttpServer, Responder,
};
use std::sync::{Mutex,RwLock,Arc};
use std::path::{PathBuf, Path};


#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(usize, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

// fn stream_movie(req: HttpRequest, db: web::Data<r2d2::Pool<SqliteConnectionManager>>) -> actix_web::Result<actix_files::NamedFile> {
//     let id: String = req.match_info().query("id").parse().unwrap();
//     let path: String = match db::get_movie_path(&db.get().unwrap(), &id) {
//         Ok(path) => path,
//         Err(error) => {
//             // TODO print error
//             return Err("");
//         }
//     };
//     Ok(actix_files::NamedFile::open(path)?)
// }


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
