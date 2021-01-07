mod docs;

use actix_web::{web};
use actix_web::web::ServiceConfig;

/// Main application routing switchboard
#[allow(non_snake_case)]
pub fn router(AppConfig: &mut ServiceConfig) {
  AppConfig
    .service(web::resource("/doc").to(docs::test))
  ;
}