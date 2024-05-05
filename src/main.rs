use tide::prelude::*;
use tide_tracing::TraceMiddleware;
use serde::Deserialize;

extern crate tracing;


mod models;
mod router;
mod handlers;
mod common;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let mut app = tide::new();
     
    Ok(run().await?)
}

async fn run()-> tide::Result<()>{
    let mut app = tide::new();

    app.with(TraceMiddleware::new());

    router::setup_routes(&mut app);

    app.listen("127.0.0.1:8080").await?;
    Ok(())


}


