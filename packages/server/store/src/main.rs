use fileplace_store::configuration::get_configuration;
use fileplace_store::startup::Application;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
