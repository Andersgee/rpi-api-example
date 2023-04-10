use actix::Actor;
use actix::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
mod valve;

#[derive(Serialize)]
struct Info {
    message: String,
}

#[derive(Deserialize)]
pub struct Authquery {
    secret: String,
}

#[get("/toggle")]
pub async fn toggle(
    valve_addr: web::Data<Addr<valve::Valve>>,
    query: web::Query<Authquery>,
) -> impl Responder {
    if query.secret != std::env::var("ADMIN_KEY").unwrap() {
        HttpResponse::Forbidden().json(Info {
            message: String::from("nope"),
        })
    } else {
        let addr = valve_addr.get_ref();
        let _res = addr.send(valve::ToggleValveMessage { ms: 1000 }).await;

        HttpResponse::Ok().json(Info {
            message: String::from("ok"),
        })
    }
}

#[get("/stuff2")]
pub async fn stuff2(query: web::Query<Authquery>) -> impl Responder {
    if query.secret != std::env::var("ADMIN_KEY").unwrap() {
        HttpResponse::Forbidden().json(Info {
            message: String::from("nope"),
        })
    } else {
        HttpResponse::Ok().json(Info {
            message: String::from("ok"),
        })
    }
}
#[get("/kek")]
pub async fn kek(query: web::Query<Authquery>) -> impl Responder {
    if query.secret != std::env::var("ADMIN_KEY").unwrap() {
        HttpResponse::Forbidden().json(Info {
            message: String::from("nope"),
        })
    } else {
        HttpResponse::Ok().json(Info {
            message: String::from("ok"),
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let api_port = std::env::var("API_PORT").expect("expected API_PORT to exists in environment");
    let _admin_key =
        std::env::var("ADMIN_KEY").expect("expected ADMIN_KEY to exists in environment");

    let valve_addr = valve::Valve::new().start();

    println!("listening on port {}", &api_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(valve_addr.clone()))
            .service(stuff2)
            .service(kek)
            .service(toggle)
    })
    .bind(format!("0.0.0.0:{}", api_port))?
    .run()
    .await
}
