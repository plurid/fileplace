use crate::helpers::spawn_app;



#[tokio::test]
async fn file_does_not_exist() {
    let app = spawn_app().await;

    let response = app.exists("one", "two").await;

    assert_eq!(
        404,
        response.status().as_u16(),
        "The API did not fail with 404 Not Found",
    );
}
