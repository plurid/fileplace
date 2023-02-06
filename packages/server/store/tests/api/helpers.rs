use urlencoding::encode;
use reqwest::multipart;

use fileplace_store::configuration::get_configuration;
use fileplace_store::startup::Application;



pub const TEST_FILE_PNG: &str = "./tests/api/assets/file.png";


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

    pub async fn metadata(
        &self,
        place: &str,
        name: &str,
    ) -> reqwest::Response {
        let address = &format!(
            "{}/metadata?place={}&name={}",
            &self.address, encode(place), encode(name),
        );

        self.api_client
            .get(address)
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn store(
        &self,
        place: &str,
        name: &str,
        file: &str,
    ) -> reqwest::Response {
        let mime_str = "image/png";
        let file_part = multipart::Part::bytes(
                std::fs::read(file).unwrap(),
            )
            .file_name(name.clone().to_owned())
            .mime_str(mime_str)
            .unwrap();
        let form = multipart::Form::new()
            .part("files[]", file_part);

        let address = &format!(
            "{}/store?place={}&name={}",
            &self.address, encode(place), encode(name),
        );

        self.api_client
            .post(address)
            .multipart(form)
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
