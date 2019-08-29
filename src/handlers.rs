use actix_web::{web, HttpResponse, Error};
use futures::Future;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde_json::json;

use crate::db::{get_anon, get_work};

pub fn get_num(
    // id: Identity,
    db: web::Data<Pool<PostgresConnectionManager>>,
    path: web::Path<(&str, i64)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // let a = check_auth(id);
    web::block(move || {
        // a?;
        let conn = db.get().unwrap();
        match path.0 {
            "anon" => Ok(get_anon(&conn, &path.1)),
            "work" => Ok(get_work(&conn, &path.1)),
            _ => Err("bad path".to_string()),
        }
    })
    .then(|res| match res {
        Ok(list) => Ok(HttpResponse::Ok().json(json!(list))),
        Err(err) => Ok(HttpResponse::Ok().json(json!(err.to_string()))),
    })
}