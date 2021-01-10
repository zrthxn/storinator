use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

use super::Request;
use crate::{db, db::DataStore};
use crate::api;

pub async fn test(req: web::Json<Request>, data: web::Data<DataStore>) -> impl Responder {
  let start = std::time::Instant::now();
  let sequence = api::parse(req.query.as_ref());
  let results = db::execute(sequence, &data.store);
  
  println!("{:?} elapsed in total", start.elapsed());
  HttpResponse::Ok().json(
    json!({ "results": results.val })
  )
}