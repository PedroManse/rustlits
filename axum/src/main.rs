mod views;
use axum::{
	routing::{get},
	Router,
	//debug_handler,
};

use tower_http::services::ServeDir;
#[tokio::main]
async fn main() -> Result<(), String> {
	let app = Router::new()
		.route("/", get(views::index))
		.route("/h1", get(views::heading_1))
		.route("/h2", get(views::heading_2))
		.route("/h3", get(views::heading_3))
		.route("/h4", get(views::heading_4))
		.nest_service("/files", ServeDir::new("files"));

	let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, app).await.unwrap();
	unreachable!();
}
