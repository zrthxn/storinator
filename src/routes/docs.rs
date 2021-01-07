use crate::db;
use actix_web::{HttpRequest, HttpResponse};

pub async fn test(_req: HttpRequest) -> HttpResponse {
  db::read::read_db();
  HttpResponse::Ok().body("Test Successful")
}