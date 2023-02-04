use fileplace_store::configuration::get_configuration;
use fileplace_store::startup::Application;
use fileplace_store::telemetry::{get_subscriber, init_subscriber};



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("fileplace-store".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
