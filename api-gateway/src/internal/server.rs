use std::net::SocketAddr;

use axum::handler::get;
use axum::Router;
use hyper::StatusCode;

use crate::config::Config;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) {
        let addr: SocketAddr = format!("{}:{}", &self.config.server.host, &self.config.server.port)
            .parse()
            .unwrap();

        let app = Router::new().route("/healthz", get(healthz));
        println!("listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

async fn healthz() -> StatusCode {
    StatusCode::OK
}
