use actix_web::{HttpRequest, HttpResponse};

pub async fn test(_req: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().body("Test Successful")
}