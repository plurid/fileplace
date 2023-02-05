use std::fs;
use std::path::{Path, PathBuf};
use actix_web::{web, Error};
use actix_web::http::header::ContentType;
use serde_json::{json, Value};

use crate::routes::utils::{
    AllQueryData,
    extract_query_params,
    METADATA_ENDING,
};



#[tracing::instrument(
    name = "all",
    skip(query, data_path),
)]
/// Returns all files from place based on pagination
pub async fn all(
    query: web::Query<AllQueryData>,
    data_path: web::Data<String>,
) -> Result<web::Json<Value>, Error> {
    let place = query.place.clone();
    let owner = query.owner.clone().unwrap_or(String::new());
    let cursor = query.cursor.clone().unwrap_or(0);

    let path;
    if owner.is_empty() {
        path = Path::new(data_path.as_str())
            .join(place)
    } else {
        path = Path::new(data_path.as_str())
            .join(owner)
            .join(place)
    }

    let mut files: Vec<String> = Vec::new();
    for file in fs::read_dir(path).unwrap() {
        let os_string = file.unwrap().file_name();
        let name = os_string.to_str().unwrap().to_string();

        if !name.ends_with(METADATA_ENDING) {
            files.push(os_string.to_str().unwrap().to_string());
        }
    }

    let value = json!({
        "files": files,
    });

    Ok(web::Json(value.clone()))
}
