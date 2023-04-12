mod valve;

use actix::{Actor, Addr};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpenvalveQuery {
    secret: String,
    ms: u64,
}

#[get("/openvalve")]
pub async fn openvalve(
    valve_addr: web::Data<Addr<valve::Valve>>,
    query: web::Query<OpenvalveQuery>,
) -> impl Responder {
    let authed = query.secret != std::env::var("SECRET").unwrap();
    match authed {
        false => HttpResponse::Forbidden(),
        true => {
            let addr = valve_addr.get_ref();
            let res = addr.send(valve::ToggleValveMessage { ms: query.ms }).await;
            match res {
                Ok(_) => HttpResponse::Ok(),
                Err(_) => HttpResponse::InternalServerError(),
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let api_port = std::env::var("API_PORT").expect("expected API_PORT to exists in environment");
    let _secret = std::env::var("SECRET").expect("expected SECRET to exists in environment");
    let valve_addr = valve::Valve::new().start();

    println!("server listening on port {}", &api_port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(valve_addr.clone()))
            .service(openvalve)
    })
    .bind(format!("0.0.0.0:{}", api_port))?
    .run()
    .await
}
