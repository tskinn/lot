use actix_web::client::Client;


const BASE_URL: &'static str = "https://api.themoviedb.org/3";


pub trait tMDB {
    pub fn SearchMovies() -> Result<Movie>
    pub fn SearchTV() -> Result<TV>
    pub fn GetMovie() -> Result<Movie> 
}




#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Results<T> {
    pub results: Vec<T>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Video {
    pub id: String,
    pub iso_639_1: String,
    pub key: String,
    pub name: String,
    pub site: String,
    pub size: u16,
    #[serde(rename = "type")]
    pub video_type: String,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Cast {
    pub id: u64,
    pub cast_id: u64,
    pub credit_id: String,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u8,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct TVCast {
    pub id: u64,
    pub credit_id: String,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u32,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct TVCreator {
    pub id: u64,
    pub credit_id: String,
    pub name: String,
    pub gender: Option<u8>,
    pub profile_path: Option<String>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Crew {
    pub credit_id: String,
    pub department: String,
    pub gender: Option<u8>,
    pub id: u64,
    pub job: String,
    pub name: String,
    pub profile_path: Option<String>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Credits {
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct TVCredits {
    pub cast: Vec<TVCast>,
    pub crew: Vec<Crew>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct LastEpisode {
    pub air_date: String,
    pub episode_number: u32,
    pub id: u64,
    pub name: String,
    pub production_code: Option<String>,
    pub season_number: u32,
    pub show_id: u64,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct ProductionCompany {
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Network {
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Season {
    pub air_date: Option<String>,
    pub episode_count: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub season_number: u32,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Movie {
    pub id: u64,
    pub imdb_id: String,
    pub title: String,
    pub tagline: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub runtime: u32,
    pub homepage: Option<String>,
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub budget: u64,
    pub adult: bool,
    pub videos: Option<Results<Video>>,
    pub credits: Option<Credits>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct TV {
    pub id: u64,
    pub backdrop_path: Option<String>,
    pub created_by: Vec<TVCreator>,
    pub episode_run_time: Vec<u64>,
    pub first_air_date: String,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub in_production: bool,
    pub languages: Vec<String>,
    pub last_air_date: String,
    pub last_episode_to_air: Option<LastEpisode>,
    pub name: String,
    pub networks: Vec<Network>,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<ProductionCompany>,
    pub seasons: Vec<Season>,
    pub status: String,
    pub r#type: String,
    pub vote_average: f64,
    pub vote_count: u64,
    pub videos: Option<Results<Video>>,
    pub credits: Option<TVCredits>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct SearchMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub adult: bool,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct FindMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub adult: bool,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct SearchResult {
    pub page: u8,
    pub total_results: u8,
    pub total_pages: u8,
    pub results: Vec<SearchMovie>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct FindResult {
    pub movie_results: Vec<FindMovie>,
}

