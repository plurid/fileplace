use actix_web::{web, Error};
use serde_json::Value;
use std::fs;
use std::path::Path;

use crate::routes::utils::{
    QueryData,
    ParsedQueryData,
    extract_query_params,
    METADATA_ENDING,
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

    let data_path = data_path.as_str();
    let address;
    if owner.trim().is_empty() {
        address = Path::new(data_path)
            .join(place).join(name);
    } else {
        address = Path::new(data_path)
            .join(owner).join(place).join(name);
    }

    let mut metadata_path = address.clone().into_os_string().into_string().unwrap();
    metadata_path.push_str(METADATA_ENDING);

    let data: String = String::from_utf8_lossy(&fs::read(metadata_path)?).parse()?;
    let value: Value = serde_json::from_str(&data)?;

    Ok(web::Json(value.clone()))
}
