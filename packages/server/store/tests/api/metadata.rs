use serde_json::Value;

use crate::helpers::{spawn_app, TEST_FILE_PNG};



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

    let body = response.text().await.unwrap().to_owned();
    let json: Value = serde_json::from_str(body.as_str()).unwrap();

    let size = json.get("size").and_then(Value::as_i64).unwrap();
    assert_eq!(
        size,
        99007,
    );

    let _ = std::fs::remove_file("./data/metadata/one/two.png");
    let _ = std::fs::remove_file("./data/metadata/one/two.png.metadata");
}
