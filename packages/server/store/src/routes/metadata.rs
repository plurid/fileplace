use actix_web::{web, HttpResponse, Error};
use serde_json::Value;
use std::fs;
use std::path::Path;

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
) -> Result<web::Json<Value>, Error> {
    let ParsedQueryData {
        place,
        name,
        owner,
    } = extract_query_params(query);

    let address = Path::new(data_path.as_str())
        .join(place)
        .join(name);
    let data: String = String::from_utf8_lossy(&fs::read(address)?).parse()?;
    let value: Value = serde_json::from_str(&data)?;

    Ok(web::Json(value.clone()))
}
