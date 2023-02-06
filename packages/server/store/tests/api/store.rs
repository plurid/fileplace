use crate::helpers::spawn_app;



const TEST_FILE_PNG: &str = "./tests/api/assets/file.png";


#[tokio::test]
async fn store_file() {
    let app = spawn_app().await;

    let response = app.store("store/one", "two.png", TEST_FILE_PNG).await;

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not succeed with 200 OK",
    );

    let _ = std::fs::remove_file("./data/store/one/two.png");
    let _ = std::fs::remove_file("./data/store/one/two.png.metadata");
}
