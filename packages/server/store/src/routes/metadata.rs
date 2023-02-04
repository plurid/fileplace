use actix_web::{web, HttpResponse};

use crate::routes::utils::{
    QueryData,
    ParsedQueryData,
    extract_query_params,
};



#[tracing::instrument(
    name = "get metadata",
    skip(query, data_path),
)]
pub async fn metadata(
    query: web::Query<QueryData>,
    data_path: web::Data<String>,
) -> HttpResponse {
    let ParsedQueryData {
        place,
        name,
        owner,
    } = extract_query_params(query);

    HttpResponse::Ok().into()
}
