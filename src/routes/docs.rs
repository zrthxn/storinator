use crate::db;
use crate::api;
use actix_web::{HttpRequest, HttpResponse};

pub async fn test(_req: HttpRequest) -> HttpResponse {
  db::read::read_db();
  api::execute();
  HttpResponse::Ok().body("Test Successful")
}