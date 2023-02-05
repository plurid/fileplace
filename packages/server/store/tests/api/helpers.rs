use urlencoding::encode;

use fileplace_store::configuration::get_configuration;
use fileplace_store::startup::Application;



pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}


impl TestApp {
    pub async fn exists(
        &self,
        place: &str,
        name: &str,
    ) -> reqwest::Response {
        self.api_client
            .get(&format!(
                "{}/exists?place={}&name={}",
                &self.address, encode(place), encode(name),
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}


pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.application.port = 0;
        c
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();

    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        api_client: client,
    };

    test_app
}
