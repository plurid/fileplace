use actix_web::web;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub place: String,
    pub name: String,
    pub owner: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreQueryData {
    pub place: String,
    pub name: Option<String>,
    pub owner: Option<String>,
}

pub struct ParsedQueryData {
    pub place: String,
    pub name: String,
    pub owner: String,
}



pub fn extract_query_params(
    query: web::Query<QueryData>,
) -> ParsedQueryData {
    let place = query.place.clone();
    let name = query.name.clone();
    let owner = query.owner.clone().unwrap_or(String::new());

    ParsedQueryData {
        place,
        name,
        owner,
    }
}


pub fn extract_store_query_params(
    query: web::Query<StoreQueryData>,
) -> ParsedQueryData {
    let place = query.place.clone();
    let name = query.name.clone().unwrap_or(String::new());
    let owner = query.owner.clone().unwrap_or(String::new());

    ParsedQueryData {
        place,
        name,
        owner,
    }
}
