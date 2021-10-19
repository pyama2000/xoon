mod config;
mod internal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::new();

    let server = internal::server::Server::new(config);
    server.run().await;

    Ok(())
}
