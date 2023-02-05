use std::fs;
use actix_web::{web, HttpResponse};
use actix_web::http::header::ContentType;

use crate::routes::utils::{
    QueryData,
    extract_query_params,
    compose_file_path,
};



#[tracing::instrument(
    name = "get",
    skip(query, data_path),
)]
pub async fn get(
    query: web::Query<QueryData>,
    data_path: web::Data<String>,
) -> HttpResponse {
    let filepath = compose_file_path(
        extract_query_params(query), data_path,
    );

    if !filepath.exists() {
        return HttpResponse::NotFound().into();
    }

    // TODO: read as stream
    let data = fs::read(filepath).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .body(data)
}
