use actix_web::HttpResponse;
use actix_easy_multipart::tempfile::Tempfile;
use actix_easy_multipart::MultipartForm;



#[derive(MultipartForm)]
pub struct Upload {
    #[multipart(rename="files[]")]
    pub files: Vec<Tempfile>,
}


pub async fn store(form: MultipartForm<Upload>) -> HttpResponse {
    dbg!("{}", &form.files[0].file);
    dbg!("{}", &form.files[0].file_name);

    HttpResponse::Ok().into()
}
