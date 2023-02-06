use crate::helpers::{spawn_app, TEST_FILE_PNG};



#[tokio::test]
async fn remove_file() {
    let app = spawn_app().await;

    let _ = app.store("remove/one", "two.png", TEST_FILE_PNG).await;

    let response = app.remove("remove/one", "two.png").await;
    let status = response.status().as_u16();

    assert_eq!(
        200,
        status,
        "The API did not succeed with 200 OK",
    );
}
