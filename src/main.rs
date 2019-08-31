use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use r2d2_postgres::PostgresConnectionManager;

use crate::handlers::get_num;

mod db;
mod handlers;

fn main() {
    dotenv().ok();
    let sys = actix_rt::System::new("prui");
    let server = dotenv::var("SERVER").expect("SERVER must be set");
    let pg = dotenv::var("PG").expect("PG must be set");
    let manager = PostgresConnectionManager::new(pg, r2d2_postgres::TlsMode::None)
        .expect("failed create connection manager");
    let pool = r2d2::Pool::new(manager).expect("error create r2d2 pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/{name}/{num}").route(web::get().to_async(get_num)))
    })
    .bind(server)
    .unwrap()
    .start();
    let _ = sys.run();
}
