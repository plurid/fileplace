use actix_web::HttpResponse;
use actix_web::http::header::ContentType;



pub async fn get(
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
                /get
            "#,
        ))
}
