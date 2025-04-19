use color_eyre::Result;

use crate::app::App;

mod app;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run().await?;
    Ok(())
}
