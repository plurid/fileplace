use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use actix_web::body::MessageBody;
use actix_web::cookie::time::ext::NumericalDuration;
use actix_web::{web, Result, HttpResponse};
use actix_easy_multipart::tempfile::Tempfile;
use actix_easy_multipart::MultipartForm;
use chrono::prelude::Utc;
use serde_json::json;
use mime_guess::get_mime_extensions;
use uuid::Uuid;



#[derive(MultipartForm)]
pub struct Upload {
    #[multipart(rename="files[]")]
    pub files: Vec<Tempfile>,
}


pub async fn write_metadata(
    path: PathBuf,
    size: usize,
) -> Result<(), anyhow::Error> {
    let mut metadata_path = path.to_str().unwrap()
        .to_owned();
    let metadata_ending: &str = ".metadata";
    metadata_path.push_str(metadata_ending);

    let mut file = fs::File::create(metadata_path.clone())?;

    let stored_at = Utc::now().timestamp().milliseconds().as_seconds_f64();
    let value = json!({
        "storedAt": stored_at,
        "size": size,
    });

    let bytes = value.as_str().unwrap().as_bytes();
    file.write_all(bytes)?;

    Ok(())
}


#[tracing::instrument(
    name = "store",
    skip(form, data_path),
)]
pub async fn store(
    form: MultipartForm<Upload>,
    data_path: web::Data<String>,
) -> HttpResponse {
// ) -> Result<HttpResponse, anyhow::Error> {
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
    let filename = Uuid::new_v4();
    let filename_ext = format!("{}.{}", filename, content_type.clone());
    let path = Path::new(data_path.as_str())
        .join(filename_ext);

    let size = temp_file.size;

    // write_metadata(
    //     path.clone(), size,
    // ).await?;

    match fs::rename(
        temp_path,
        path,
    ) {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
        // Ok(_) => Ok(HttpResponse::Ok().into()),
        // Err(_) => Err(HttpResponse::BadRequest().into()),
    }
}
