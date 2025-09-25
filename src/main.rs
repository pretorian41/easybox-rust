 mod models;
 mod packer;
// use tokio::net::{TcpListener, TcpStream};
// use mini_redis::{Connection, Frame};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use cfg_if::cfg_if;

// use gelf_logger::{GelfLogger, GelfLoggerBuilder};
use log::{info, LevelFilter};

mod logger;
// use log::Level;
use models::item::Item;
use models::basebox::BoxConstraints;
use packer::packer::pack_items;
// use start::logger;

// #[tokio::main]
// async fn main() {
//     simple_logger::init_with_level(Level::Trace).expect("logger init failed");
//     println!("Listening");
//     info!("Сервіс запущено");
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger(
        LevelFilter::Trace,
        LevelFilter::Info,
        Some("127.0.0.1:12201"), // адреса Graylog UDP input
    );
    println!("Listening");
    info!("Сервіс запущено");
    HttpServer::new(|| {
        App::new()
            // .data(web::JsonConfig::default().limit(4096)) 
            .service(hello)
            .service(pack)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/pack")]
async fn pack(
    web::Json(items): web::Json<Vec<Item>>,
) -> impl Responder {
    let constraints = BoxConstraints {
        max_width: 200,
        max_height: 300,
        max_depth: 200,
        max_weight: 500,
        max_price: 1000,
    };

    let packed_boxes = pack_items(items, constraints);

    HttpResponse::Ok().json(packed_boxes)
}


async fn manual_hello() -> impl Responder {

    info!("hello");
    HttpResponse::Ok().body("Hey there!")
}
