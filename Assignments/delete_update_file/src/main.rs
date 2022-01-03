use actix_web::{web, App, HttpServer};
use handler::handlers::{create_file, delete_file, rename_file};
pub mod test;
pub mod handler {
    pub mod handlers;
}
/// main function is used to create a new HTTP Server
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Returns std::io::Result<()>
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    log::info!("Starting a HTTP Server");
    HttpServer::new(|| {
        App::new()
            .route("/create_file", web::post().to(create_file))
            .route("/rename_file", web::put().to(rename_file))
            .route("/delete_file", web::delete().to(delete_file))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
