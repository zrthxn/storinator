use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[get("/{name}")]
pub async fn helloname(req: HttpRequest) -> impl Responder {
  HttpResponse::Ok().body(
    format!("Hello {}!", req.match_info().get("name").unwrap_or("World"))
  )
}