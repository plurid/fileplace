use actix_web::{web, HttpResponse};
use actix_web::http::header::ContentType;

use crate::routes::utils::{
    QueryData,
    ParsedQueryData,
    extract_query_params,
};



pub async fn get(
    query: web::Query<QueryData>,
) -> HttpResponse {
    let ParsedQueryData {
        place,
        name,
        owner,
    } = extract_query_params(query);

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
                /get {place} {name} {owner}
            "#,
        ))
}
