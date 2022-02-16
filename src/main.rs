use actix_web::{get, web, App, HttpServer, Responder, middleware::Logger};
use serde_json::json;
use std::env;
use chrono::{DateTime,Local};

struct AppState {
    start_time: DateTime<Local>
}

#[get("/")]
async fn index() -> impl Responder {
    "It works! (and it's Blazingly Fast ^_^)"
}

#[get("/health")]
async fn healthcheck(data: web::Data<AppState>) -> impl Responder {
    let now = Local::now();

    /*
     * Note: Here we can perform checks to resolve if our instance is good 
     * or not. A good example of that is checking Database Connection
     */

    web::Json(json!({
        "now": format!("{}", now.to_rfc2822()),
        "start-time": data.start_time.to_rfc2822(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger implementation
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("info")
    );

    let addr = match env::var("BIND_ADDR") {
        Ok(host) => host,
        Err(_e) => "0.0.0.0:8080".to_string(),
    };

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                start_time: Local::now(),
            }))
            .wrap(Logger::default())
            .service(index)
            .service(healthcheck)
    })
    .bind(addr).expect("Failed to bind Http Server address")
    .run()
    .await
}

