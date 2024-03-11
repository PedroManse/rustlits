use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::sqlite::SqlitePool;
use std::sync::Mutex;

struct AppState {
	name: String,
	pool: SqlitePool,
	counter: Mutex<i32>,
}

impl AppState {
	fn new(pool: SqlitePool) -> Self {
		let app_name: String = "Actix-Web Example!".to_owned();
		let counter = Mutex::new(0);

		AppState{
			name: app_name,
			pool,
			counter,
		}
	}

	fn app_data(self) -> web::Data<Self> {
		web::Data::new(self)
	}
}

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[get("/count")]
async fn count(
	data: web::Data<AppState>
) -> impl Responder {
	let counter = {
		let mut counter = data.counter.lock().unwrap();
		*counter += 1;
		*counter
	};
	HttpResponse::Ok().body(format!("#{counter}"))
}

#[get("/dbinfo")]
async fn dbinfo(
	data: web::Data<AppState>
) -> impl Responder {
	let pool = &data.pool;
	format!("{pool:?}")
}

#[get("/name")]
async fn name(
	data: web::Data<AppState>
) -> impl Responder {
	let name = &data.name;
	format!("Hello from {name}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenvy::dotenv().expect("Couldn't read .env file");
	let url = std::env::var("DATABASE_URL").expect("Can't find DATABASE_URL in env");
	let pool = SqlitePool::connect(&url)
		.await
		.expect("Can't use sqlite3 in memory");
	sqlx::migrate!().run(&pool).await.expect("can't migrate!");

	HttpServer::new(move ||{
		App::new()
			.app_data( AppState::new(pool.clone()).app_data() )
			.service(name)
			.service(dbinfo)
			.service(hello)
			.service(echo)
			.service(count)
			.service(info)
	})
		.bind(("0.0.0.0", 8080))?
		.run()
		.await
}
