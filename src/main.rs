use actix_web::{web, App, HttpServer, Responder};
use std::{collections::HashMap, sync::{Arc, RwLock}, time::Instant};
use tokio::sync::Notify;

mod models;
mod handlers;

type Connections = RwLock<HashMap<String, TempConnection>>;

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

struct TempConnection {
    last_activity: Instant,
}

async fn hello() -> impl Responder {
    "<h1>Hello World!!!</h1>"
}

async fn create_connection(
    id: String,
    connections: &Connections,
) {
   connections.write().unwrap().insert(//explicitly handle this even if it cant happen
    id,
    TempConnection {
        last_activity: Instant::now(), 
    }
    );

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let mut active_games: HashMap<String, Game> = HashMap::new();//hasmap containing all active games
    let connections: Connections = RwLock::new(HashMap::new());

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

            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

