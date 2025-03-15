#[macro_use] extern crate rocket;

use clap::Parser;
use rocket::fs::{FileServer, relative};
use rocket::tokio::sync::mpsc;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct AppState {
    // Add any shared state here
}

#[get("/")]
async fn index() -> &'static str {
    "Welcome to NightE"
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .manage(Arc::new(Mutex::new(AppState {})))
}

pub async fn webStart() -> bool {
    let _rocket = rocket().await; // Actually launch the rocket
    true // Or return the rocket instance if you need it elsewhere
}

#[tokio::main]
async fn main() {
    if webStart().await {
        println!("Web server started successfully.");
    } else {
        println!("Failed to start web server.");
    }
}
