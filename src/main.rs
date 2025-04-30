use color_eyre::Result;
use tanki::app::App;
use tracing::{Level, debug, info};
use tracing_appender::rolling;
use tracing_subscriber::fmt::SubscriberBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = rolling::daily("logs", "app.log");
    let subscriber = SubscriberBuilder::default().with_max_level(Level::DEBUG).with_writer(file_appender).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
    info!("Starting Tanki application");

    let mut app = App::new()?;
    app.run().await?;
    Ok(())
}
