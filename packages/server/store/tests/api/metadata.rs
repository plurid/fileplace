use serde_json::Value;

use crate::helpers::{
    TEST_FILE_PNG,
    spawn_app,
    get_json,
};



#[tokio::test]
async fn gets_metadata() {
    let app = spawn_app().await;

    let _ = app.store("metadata/one", "two.png", TEST_FILE_PNG).await;
    let response = app.metadata("metadata/one", "two.png").await;
    let status = response.status().as_u16();

    assert_eq!(
        200,
        status,
        "The API did not succeed with 200 OK",
    );

    let json = get_json(response).await;

    let size = json.get("size").and_then(Value::as_i64).unwrap();
    assert_eq!(
        99007,
        size,
    );

    let _ = std::fs::remove_file("./data/metadata/one/two.png");
    let _ = std::fs::remove_file("./data/metadata/one/two.png.metadata");
}
