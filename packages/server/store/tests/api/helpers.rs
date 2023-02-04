use fileplace_store::configuration::get_configuration;
use fileplace_store::startup::Application;



pub struct TestApp {
    pub address: String,
    pub port: u16,
}


pub async fn spawn_app() -> TestApp {
    let configuration = {
        let c = get_configuration().expect("Failed to read configuration.");
        c
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();

    let _ = tokio::spawn(application.run_until_stopped());

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
    };

    test_app
}
