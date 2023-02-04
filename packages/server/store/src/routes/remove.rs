use actix_web::{web, HttpResponse};

use crate::routes::utils::{
    QueryData,
    ParsedQueryData,
    extract_query_params,
};



pub async fn remove(
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
