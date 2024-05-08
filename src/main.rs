use tide::prelude::*;
use tide::Server;

mod common;
mod handlers;
mod models;
mod router;

#[async_std::main]
async fn main() -> tide::Result<()> {
    
    let mut app = tide::new();

    // logging functinality
    // for developers 
    app.with(driftwood::DevLogger);
    //for deployed applications
    //app.with(driftwood::ApacheCombinedLogger);

    Ok(run(&mut app).await?)
}

/// Run function this starts the server
async fn run(app: &mut Server<()>) -> tide::Result<()> {
    router::setup_routes(app);

    // same as '''app.listen("127.0.0.1:8080").await?;'''  but
    <Server<()> as Clone>::clone(&app)
        .listen("127.0.0.1:8080")
        .await?;

    Ok(())
}
