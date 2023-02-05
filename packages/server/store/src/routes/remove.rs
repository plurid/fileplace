use std::fs;
use std::path::Path;
use actix_web::{web, HttpResponse};

use crate::routes::utils::{
    QueryData,
    extract_query_params,
    compose_file_path,
    compose_metadata_path,
};



#[tracing::instrument(
    name = "remove",
    skip(query, data_path),
)]
pub async fn remove(
    query: web::Query<QueryData>,
    data_path: web::Data<String>,
) -> HttpResponse {
    let path = compose_file_path(
        extract_query_params(query),
        data_path,
    );

    let metadata_file = compose_metadata_path(path.clone());
    let metadata_path = Path::new(metadata_file.as_str());

    if !path.exists() || !metadata_path.exists() {
        return HttpResponse::NotFound().into();
    }
    let _ = fs::remove_file(path.clone());
    let _ = fs::remove_file(metadata_path.clone());

    HttpResponse::Ok().into()
}
