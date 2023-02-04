use actix_web::web;



#[derive(serde::Deserialize)]
pub struct QueryData {
    pub place: String,
    pub name: String,
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
    let owner = match query.owner.clone() {
        Some(value) => value,
        None => String::new(),
    };

    ParsedQueryData {
        place,
        name,
        owner,
    }
}
