use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::io;
use actix_web::{web, Error};
use actix_web::http::header::ContentType;
use serde_json::{json, Value};
use walkdir::WalkDir;

use crate::routes::utils::{
    AllQueryData,
    extract_query_params,
    METADATA_ENDING,
};



#[tracing::instrument(
    name = "places",
    skip(data_path),
)]
pub async fn places(
    data_path: web::Data<String>,
) -> Result<web::Json<Value>, Error> {
    let path = Path::new(data_path.as_str());

    let mut places: Vec<String> = vec![];

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let value = path.to_owned().into_os_string().into_string().unwrap()
                .replace(data_path.as_str(), "");

            if !value.trim().is_empty() {
                let place = value
                    .chars()
                    .skip(1)
                    .collect::<String>();

                    places.push(place);
            }
        }
    }

    places.sort();

    let value = json!({
        "status": true,
        "places": places,
    });

    Ok(web::Json(value.clone()))
}
