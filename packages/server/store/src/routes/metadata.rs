use actix_web::HttpResponse;



pub async fn metadata(
) -> HttpResponse {
    HttpResponse::Ok().into()
}
