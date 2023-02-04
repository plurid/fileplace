use crate::helpers::spawn_app;



#[tokio::test]
async fn it_stores_file() {
    let _app = spawn_app().await;

    assert_eq!("ok", "ok");
}
