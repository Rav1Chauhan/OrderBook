use crate::routes::{create_order, delete_order, get_depth};
use actix_web::{App, HttpServer};
pub mod inputs;
pub mod outputs;
pub mod routes;
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
