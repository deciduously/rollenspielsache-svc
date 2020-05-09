//! rollenspielsache-svc is a backend web service for working with [librollenspielsache](https://docs.rs/librollenspielsache) types

use actix_service::Service;
use actix_web::{
    http::{header::*, HeaderValue},
    App, HttpServer,
};
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
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                // CORS headers
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    res.headers_mut().insert(
                        ACCESS_CONTROL_ALLOW_ORIGIN,
                        HeaderValue::from_static("https://rollenspielsache.deciduously.com"),
                    );
                    res.headers_mut()
                        .insert(VARY, HeaderValue::from_static("Origin"));
                    Ok(res)
                }
            })
            .service(index)
            .service(pong)
            .service(roll)
    });
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
