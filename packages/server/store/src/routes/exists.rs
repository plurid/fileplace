use actix_web::{web, HttpResponse};
use std::path::{Path, PathBuf};

use crate::routes::utils::{
    QueryData,
    ParsedQueryData,
    extract_query_params,
};



pub async fn exists(
    query: web::Query<QueryData>,
    data_path: web::Data<String>,
) -> HttpResponse {
    let ParsedQueryData {
        place,
        name,
        owner,
    } = extract_query_params(query);

    let path;
    if owner.is_empty() {
        path = Path::new(data_path.as_str())
            .join(place)
            .join(name);
    } else {
        path = Path::new(data_path.as_str())
            .join(owner)
            .join(place)
            .join(name);
    }

    if !path.exists() {
        return HttpResponse::NotFound().into();
    }

    HttpResponse::Ok().into()
}
