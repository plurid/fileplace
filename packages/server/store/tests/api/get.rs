use std::fs::File;
use std::io::Write;

use crate::helpers::{spawn_app, TEST_FILE_PNG};



#[tokio::test]
async fn get_file() {
    let app = spawn_app().await;

    let _ = app.store("get/one", "two.png", TEST_FILE_PNG).await;

    let response = app.get("get/one", "two.png").await;
    let status = response.status().as_u16();

    assert_eq!(
        200,
        status,
        "The API did not succeed with 200 OK",
    );

    let get_file_path = "./tests/api/assets/get-file.png";

    let mut file = File::create(get_file_path).unwrap();
    let written = file.write_all(&response.bytes().await.unwrap()).unwrap();
    assert_eq!(
        (),
        written,
        "File was not written",
    );

    let _ = std::fs::remove_file("./data/get/one/two.png");
    let _ = std::fs::remove_file("./data/get/one/two.png.metadata");
}
