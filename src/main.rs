use actix_files::NamedFile;
use actix_cors::Cors;
use actix_web::{
    client, dev::RequestHead, get, web, App, Error, HttpRequest,
    HttpServer, Responder, HttpResponse
};
use actix_web::body::Body;
use std::sync::{Mutex,RwLock,Arc};
use std::path::{PathBuf, Path};
use http::StatusCode;
use clap;
use std::borrow::Borrow;
use std::borrow::Cow;
use rust_embed::RustEmbed;

mod store;
mod models;

use models::{VideoStore,Data};

async fn index() -> impl Responder {
    format!("check")
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

fn player_site() -> HttpResponse {
		match Asset::get("index.html") {
				Some(content) => {
						let body: Body = match content {
								Cow::Borrowed(bytes) => bytes.into(),
								Cow::Owned(bytes) => bytes.into(),
						};
						return HttpResponse::Ok().body(body);
				}
				None => HttpResponse::NotFound().body("404 Not Found")
		}
}

async fn counter_read(data: web::Data<Mutex<Arc<usize>>>) -> impl Responder {
    let val;
    {
        let mut thing = data.lock().unwrap();
        *thing = Arc::new(1usize + **thing);
        val = thing;
    }
    format!("{}", val)
}

async fn stream_movie(req: HttpRequest, data: web::Data<Mutex<Arc<models::JsonVideoStore>>>) -> actix_web::Result<actix_files::NamedFile> {
    let id: String = req.match_info().query("id").parse().unwrap();
    let store: Arc<models::JsonVideoStore>;
    {
        store = data.lock().unwrap().clone();
    }

    println!("streaming movie id {}", id);
    let movie = store.movies.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;
    Ok(actix_files::NamedFile::open(movie.path.clone())?)
}

async fn list_episodes(data: web::Data<Mutex<Arc<models::JsonVideoStore>>>) -> actix_web::Result<HttpResponse> {
    println!("List Episodes");
    let store: Arc<models::JsonVideoStore>;
    {
        store = data.lock().unwrap().clone();
    }
    
    let resp = Data{
        data: store.get_episode_all().unwrap()
    };
    Ok(HttpResponse::Ok().json(resp))
}

async fn list_movies(data: web::Data<Mutex<Arc<models::JsonVideoStore>>>) -> actix_web::Result<HttpResponse> {
    println!("List Movies");
    let store: Arc<models::JsonVideoStore>;
    {
        store = data.lock().unwrap().clone();
    }
    
    let resp = Data{
        data: store.get_movie_all().unwrap()
    };
    Ok(HttpResponse::Ok().json(resp))
}

async fn stream_episode(req: HttpRequest, data: web::Data<Mutex<Arc<models::JsonVideoStore>>>) -> actix_web::Result<actix_files::NamedFile> {
    let id: String = req.match_info().query("id").parse().unwrap();

    let store: Arc<models::JsonVideoStore>;
    {
        store = data.lock().unwrap().clone();
    }

    let episode = store.episodes.get(&id).ok_or(actix_web::HttpResponse::new(StatusCode::from_u16(400).unwrap()))?;
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
        .arg(clap::Arg::with_name("store")
             .short("s")
             .long("store")
             .required(true)
             .takes_value(true)
             .help("path to json file with video info")).get_matches();

    let address = matches.value_of("address").unwrap();
    let store_path = matches.value_of("path").unwrap();
    let data = match models::JsonVideoStore::from_file(&PathBuf::from(&store_path)) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            println!("Unable to load storage at: {}", store_path);
            return Ok(());
        }
    };
    let data = web::Data::new(Mutex::new(Arc::new(data)));
    let counter = web::Data::new(Mutex::new(Arc::new(0usize)));
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .send_wildcard()
                    .max_age(3600)
                    .finish())
            .app_data(data.clone())
            .app_data(counter.clone())
						.service(web::scope("/player")
										 .route("", web::get().to(player_site)))
            .service(web::scope("/movies")
                     .route("", web::get().to(list_movies))                     
                     .route("/", web::get().to(list_movies))
                     .route("/{id}", web::get().to(stream_movie)))
            .service(web::scope("/episodes")
                     .route("", web::get().to(list_episodes))
                     .route("/", web::get().to(list_episodes))
                     .route("/{id}", web::get().to(stream_episode)))
            .service(web::resource("/health").to(index))
            .service(web::resource("/test").to(counter_read))
    })
        .bind(address)?
        .run()
        .await
}
