use std::fs;
use actix_web::HttpResponse;
use actix_easy_multipart::tempfile::Tempfile;
use actix_easy_multipart::MultipartForm;
use mime_guess::get_mime_extensions;
use uuid::Uuid;



#[derive(MultipartForm)]
pub struct Upload {
    #[multipart(rename="files[]")]
    pub files: Vec<Tempfile>,
}


pub async fn store(form: MultipartForm<Upload>) -> HttpResponse {
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
    let path = format!("{}.{}", filename, content_type.clone());

    match fs::rename(
        temp_path,
        path,
    ) {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}
