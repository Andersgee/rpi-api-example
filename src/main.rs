use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use rppal::gpio::{Gpio, OutputPin};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

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
                p.set_low();
                thread::sleep(Duration::from_millis(2000));
                p.set_high()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let api_port = std::env::var("API_PORT").expect("expected API_PORT to exists in environment");
    let _admin_key =
        std::env::var("ADMIN_KEY").expect("expected ADMIN_KEY to exists in environment");

    //let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

    //configure the pin as output (always 3.3V I think?)
    //let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let pin = web::Data::new(Mutex::new(
        Gpio::new()
            .expect("expected gpio new to be fine")
            .get(GPIO_LED)
            .expect("expected to get GPIO_LED to be fine")
            .into_output_high(),
    ));

    println!("listening on port {}", &api_port);

    HttpServer::new(move || App::new().app_data(pin.clone()).service(stuff2))
        .bind(format!("0.0.0.0:{}", api_port))?
        .run()
        .await
}
