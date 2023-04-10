use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod db;
use db::*;
use dotenv::dotenv;

use rppal::gpio::{Gpio, OutputPin};
//use simple_signal::{self, Signal};
//use std::error::Error;
//use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
//use std::thread;
//use std::time::Duration;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 18;

#[derive(Serialize)]
struct Info {
    message: String,
}

#[derive(Deserialize)]
pub struct Authquery {
    secret: String,
}

// this handler gets called if the query deserializes into `Authquery` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/stuff")]
pub async fn stuff(
    client: web::Data<PrismaClient>,
    query: web::Query<Authquery>,
) -> impl Responder {
    if query.secret != std::env::var("ADMIN_KEY").unwrap() {
        HttpResponse::Forbidden().json(Info {
            message: String::from("nope"),
        })
    } else {
        let users = client.user().find_many(vec![]).exec().await.unwrap();
        HttpResponse::Ok().json(users)
    }
}

#[get("/stuff2")]
pub async fn stuff2(
    pin: web::Data<Mutex<OutputPin>>,
    query: web::Query<Authquery>,
) -> impl Responder {
    if query.secret != std::env::var("ADMIN_KEY").unwrap() {
        HttpResponse::Forbidden().json(Info {
            message: String::from("nope"),
        })
    } else {
        match pin.lock() {
            Ok(mut p) => {
                p.toggle();
            }
            _ => println!("couldnt get mutable pin from pin.lock()"),
        }
        //let mut p = pin.lock().unwrap();
        //p.toggle();

        HttpResponse::Ok().json(Info {
            message: String::from("ok"),
        })
    }
}

#[get("/users")]
async fn get_users(client: web::Data<PrismaClient>) -> impl Responder {
    //let key = std::env::var("ADMIN_KEY").unwrap();

    let users = client.user().find_many(vec![]).exec().await.unwrap();

    HttpResponse::Ok().json(users)
}

#[derive(Deserialize)]
struct CreateUserRequest {
    display_name: String,
}

#[post("/user")]
async fn create_user(
    client: web::Data<PrismaClient>,
    body: web::Json<CreateUserRequest>,
) -> impl Responder {
    let user = client
        .user()
        .create(body.display_name.to_string(), vec![])
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().json(user)
}

#[get("/posts")]
async fn get_posts(client: web::Data<PrismaClient>) -> impl Responder {
    let posts = client.post().find_many(vec![]).exec().await.unwrap();

    HttpResponse::Ok().json(posts)
}

#[derive(Deserialize)]
struct CreatePostRequest {
    content: String,
    user_id: String,
}

#[post("/post")]
async fn create_post(
    client: web::Data<PrismaClient>,
    body: web::Json<CreatePostRequest>,
) -> impl Responder {
    let post = client
        .post()
        .create(
            body.content.to_string(),
            user::id::equals(body.user_id.to_string()),
            vec![],
        )
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().json(post)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let api_port = std::env::var("API_PORT").expect("expected API_PORT to exists in environment");
    let _admin_key =
        std::env::var("ADMIN_KEY").expect("expected ADMIN_KEY to exists in environment");

    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

    //configure the pin as output (always 3.3V I think?)
    //let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let mut pin = web::Data::new(Mutex::new(
        Gpio::new()
            .expect("expected gpio new to be fine")
            .get(GPIO_LED)
            .expect("expected to get GPIO_LED to be fine")
            .into_output(),
    ));

    println!("listening on port {}", &api_port);

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .app_data(pin.clone())
            .service(get_users)
            .service(create_user)
            .service(get_posts)
            .service(create_post)
            .service(stuff)
    })
    .bind(format!("0.0.0.0:{}", api_port))?
    .run()
    .await
}
