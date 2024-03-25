use actix_web::{App, HttpServer};
use RunnyPush::services::{get_sunny_info, get_sunny_list};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_sunny_list)
            .service(get_sunny_info)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}