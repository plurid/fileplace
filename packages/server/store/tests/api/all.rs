use serde_json::Value;

use crate::helpers::{
    TEST_FILE_PNG,
    spawn_app,
    get_json,
};



#[tokio::test]
async fn get_all() {
    let app = spawn_app().await;

    let _ = app.store("all/one", "two.png", TEST_FILE_PNG).await;
    let _ = app.store("all/one", "three.png", TEST_FILE_PNG).await;

    let response = app.all("all/one").await;
    let status = response.status().as_u16();

    assert_eq!(
        200,
        status,
        "The API did not succeed with 200 OK",
    );

    let json = get_json(response).await;

    let status = json.get("status").and_then(Value::as_bool).unwrap();
    assert_eq!(
        true,
        status,
    );

    let files = json.get("files").unwrap().as_array().unwrap();
    assert_eq!(
        2,
        files.len(),
    );

    let _ = std::fs::remove_file("./data/all/one/two.png");
    let _ = std::fs::remove_file("./data/all/one/two.png.metadata");
    let _ = std::fs::remove_file("./data/all/one/three.png");
    let _ = std::fs::remove_file("./data/all/one/three.png.metadata");
}
