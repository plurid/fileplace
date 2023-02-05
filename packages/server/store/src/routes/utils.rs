use std::fs;
use std::path::{Path, PathBuf};
use actix_web::web;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub place: String,
    pub name: String,
    pub owner: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreQueryData {
    pub place: String,
    pub name: Option<String>,
    pub owner: Option<String>,
}

pub struct ParsedQueryData {
    pub place: String,
    pub name: String,
    pub owner: String,
}



pub fn extract_query_params(
    query: web::Query<QueryData>,
) -> ParsedQueryData {
    let place = query.place.clone();
    let name = query.name.clone();
    let owner = query.owner.clone().unwrap_or(String::new());

    ParsedQueryData {
        place,
        name,
        owner,
    }
}


pub fn extract_store_query_params(
    query: web::Query<StoreQueryData>,
) -> ParsedQueryData {
    let place = query.place.clone();
    let name = query.name.clone().unwrap_or(String::new());
    let owner = query.owner.clone().unwrap_or(String::new());

    ParsedQueryData {
        place,
        name,
        owner,
    }
}



pub fn compose_file_path(
    query_data: ParsedQueryData,
    data_path: web::Data<String>,
) -> PathBuf {
    let ParsedQueryData {
        owner,
        place,
        name,
    } = query_data;

    let path;
    if owner.is_empty() {
        path = Path::new(data_path.as_str())
            .join(place)
            .join(name);
    } else {
        path = Path::new(data_path.as_str())
            .join(owner)
            .join(place)
            .join(name);
    }

    path
}



pub fn make_directory(
    path: PathBuf,
) {
    let directory = path.parent().unwrap_or(Path::new("./"));
    if !directory.exists() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }
}
