use actix_web::HttpResponse;



pub async fn exists(
) -> HttpResponse {
    HttpResponse::Ok().into()
}
