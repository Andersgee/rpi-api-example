use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod db;
use db::*;
use dotenv::dotenv;

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
pub async fn stuff(client: web::Data<PrismaClient>, info: web::Query<Authquery>) -> impl Responder {
    if info.secret != std::env::var("ADMIN_KEY").unwrap() {
        HttpResponse::Forbidden().json(Info {
            message: String::from("nope"),
        })
    } else {
        let users = client.user().find_many(vec![]).exec().await.unwrap();
        HttpResponse::Ok().json(users)
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
    let admin_key =
        std::env::var("ADMIN_KEY").expect("expected ADMIN_KEY to exists in environment");

    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

    println!("listening on port {}", &api_port);

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
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
