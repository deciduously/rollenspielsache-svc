//! rollenspielsache-svc is a backend web service for working with [librollenspielsache](https://docs.rs/librollenspielsache) types

use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use log::info;

mod config;
mod handlers;

use crate::{
    config::{init_logging, OPT},
    handlers::*,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Init logging
    init_logging(2);
    // Set URL
    let addr = format!("{}:{}", OPT.address, OPT.port);
    // Set up autoreloader
    let mut listenfd = ListenFd::from_env();
    // Define server
    let mut server = HttpServer::new(|| App::new().service(index).service(pong).service(roll));
    // Catch reload
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(&addr)?
    };

    // Run it
    info!("Mounting server at {}", &addr);
    server.run().await
}
