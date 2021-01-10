use actix_web::{HttpRequest, HttpResponse, Responder, web};
use super::Request;
use crate::{db, db::DataStore};
use crate::api;

pub async fn test(req: web::Json<Request>, data: web::Data<DataStore>) -> impl Responder {  
  let sequence = api::parse(req.query.as_ref());
  let results = db::execute(&sequence, &data.store);
  
  HttpResponse::Ok().body("Test Successful")
}