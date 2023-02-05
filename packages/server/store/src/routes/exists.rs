use actix_web::{web, HttpResponse};

use crate::routes::utils::{
    QueryData,
    extract_query_params,
    compose_file_path,
};



#[tracing::instrument(
    name = "exists",
    skip(query, data_path),
)]
pub async fn exists(
    query: web::Query<QueryData>,
    data_path: web::Data<String>,
) -> HttpResponse {
    let path = compose_file_path(
        extract_query_params(query),
        data_path,
    );

    if !path.exists() {
        return HttpResponse::NotFound().into();
    }

    HttpResponse::Ok().into()
}
