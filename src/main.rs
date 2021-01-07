/**
 * @info
 */

mod config;
mod routes;
mod db;
mod api;

use actix_web::{App, HttpServer, middleware::Logger};
use color_eyre::Result;

use crate::config::Config;
use routes::router;

#[actix_web::main]
async fn main() -> Result<()> {
	let server = Config::loadenv()
		.expect("Server bind configuration");

	HttpServer::new(|| {
		App::new()
			.wrap(Logger::default())
			.configure(router)
	})
		.bind((server.host, server.port))?
		.run()
		.await?;

	Ok(())
}