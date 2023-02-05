use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;
use actix_web::{web, Result, HttpResponse};
use actix_easy_multipart::tempfile::Tempfile;
use actix_easy_multipart::MultipartForm;
use chrono::prelude::Utc;
use serde_json::json;
use mime_guess::get_mime_extensions;
use uuid::Uuid;

use crate::routes::utils::{
    StoreQueryData,
    ParsedQueryData,
    extract_store_query_params,
    compose_file_path,
    make_directory,
    compose_metadata_path,
};



#[derive(MultipartForm)]
pub struct Upload {
    #[multipart(rename="files[]")]
    pub files: Vec<Tempfile>,
}


pub async fn write_metadata(
    path: PathBuf,
    size: usize,
) -> Result<bool, anyhow::Error> {
    let metadata_path = compose_metadata_path(path.clone());
    let mut file = fs::File::create(metadata_path.clone())?;

    let stored_at = Utc::now().timestamp_millis();
    let value = json!({
        "storedAt": stored_at,
        "size": size,
    });

    let bytes_string = value.clone().to_string();
    let bytes = bytes_string.as_bytes();
    file.write_all(bytes)?;

    Ok(true)
}


#[tracing::instrument(
    name = "store",
    skip(form, query, data_path),
)]
pub async fn store(
    form: MultipartForm<Upload>,
    query: web::Query<StoreQueryData>,
    data_path: web::Data<String>,
) -> HttpResponse {
    let temp_file = &form.files[0];
    let named_temp_file = &temp_file.file;
    let temp_path =  named_temp_file.path().as_os_str();
    let content_type = match &temp_file.content_type {
        Some(val) => match get_mime_extensions(val) {
            Some(v) => {
                let mut iter = v.into_iter();
                match iter.next() {
                    Some(value) => value.to_string(),
                    None => String::new(),
                }
            },
            None => String::new(),
        },
        None => String::new(),
    };

    let ParsedQueryData {
        place,
        name,
        owner,
    } = extract_store_query_params(query);

    let filename_id = Uuid::new_v4();
    let filename_ext = format!("{}.{}", filename_id, content_type.clone());

    let filename;
    if name.is_empty() {
        filename = filename_ext;
    } else {
        filename = name.clone();
    }

    let path = compose_file_path(
        ParsedQueryData {
            place: place.clone(),
            owner: owner.clone(),
            name: filename.clone(),
        },
        data_path.clone(),
    );

    make_directory(path.clone());

    let size = temp_file.size;
    let _ = write_metadata(
        path.clone(), size,
    ).await;

    match fs::rename(
        temp_path,
        path,
    ) {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}



#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_writes_metadata() -> Result<(), anyhow::Error> {
        // Setup
        use super::*;
        use std::path::Path;

        let path = Path::new("./data/one").to_path_buf();
        let written = write_metadata(
            path.clone(), 200,
        ).await.expect("couldn't write metadata");

        // Assert
        assert_eq!(written, true);

        // Cleanup
        let metadata_path = compose_metadata_path(path.clone());
        std::fs::remove_file(metadata_path)?;

        Ok(())
    }
}
