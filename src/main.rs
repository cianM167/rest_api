use actix_web::{web, App, HttpServer, Responder};
use std::collections::HashMap;

mod models;
mod handlers;

struct Character {
    str: i8,
    dex: i8,
    con: i8,
    int: i8,
    wiz: i8,
    chr: i8,   
}

struct Game {
    characters: HashMap<String, Character>,
}

async fn hello() -> impl Responder {
    "<h1>Hello World!!!</h1>"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut active_games: HashMap<String, Game> = HashMap::new();//hasmap containing all active games


    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/items", web::post().to(handlers::create_item))
                    .route("/items", web::get().to(handlers::get_items))
                    .route("/items/{id}", web::get().to(handlers::get_item))
            )
            .service(
                web::scope("/game")
                    .wrap(Timeout::new(Duration::from_secs(5)))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

///*

async fn add_game(db_parameters) {

}

//*/