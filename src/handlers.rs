//! The Handlers module defines the endpoints

use actix_web::{get, http, web, HttpResponse, Responder, Result};
use log::info;
use rollenspielsache::dice::Roll;
use std::str::FromStr;

/// Placeholder index - all usage should be via one of the other endpoints
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Found()
        .header(
            http::header::LOCATION,
            "https://rollenspielsache-www.now.sh/",
        )
        .finish()
        .into_body()
}

/// Just confirms a valid connection
#[get("/ping")]
async fn pong() -> impl Responder {
    info!("ping");
    HttpResponse::Ok().body("pong")
}

/// Roll a die from a string
#[get("/roll/{input}")]
async fn roll(info: web::Path<String>) -> Result<HttpResponse> {
    info!("rolling {}...", &info);
    Ok(HttpResponse::Ok().json(Roll::from_str(&info).unwrap().execute()))
}
