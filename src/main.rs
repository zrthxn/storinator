/**
 * @info
 */

mod db;
mod api;
mod configs;
mod routes;

use std::sync::Mutex;

use actix_web::{App, HttpServer, middleware::Logger, web};
use color_eyre::Result;

use db::DataStore;
use configs::Config;
use crate::routes::router;

#[actix_web::main]
async fn main() -> Result<()> {
	let server = Config::loadenv()
		.expect("Server configuration missing");

	let data = std::fs::read_to_string("store.json")
    .expect("Error in reading datastore");

	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(
				DataStore {
					store: Mutex::new(serde_json::from_str(&data).unwrap())
				}
			).clone())
			.wrap(Logger::default())
			.configure(router)
	})
		.bind((server.host, server.port))?
		.run()
		.await?;

	Ok(())
}