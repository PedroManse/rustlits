const ARGON_SALT: &[u8] = b"example salt TODO: make unique";

// String.as_bytes -> [u8]
pub fn hash_password(pass: &[u8]) -> Result<Vec<u8>, argon2::Error> {
	let mut output_key_material = vec![0u8; 32];
	argon2::Argon2::default()
		.hash_password_into(pass, ARGON_SALT, &mut output_key_material)?;
	Ok(output_key_material)
}

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
pub fn establish_connection() -> Result<PgConnection, String> {
	dotenv().map_err(|e|format!("can't read .env file: {e}"))?;

	let database_url = std::env::var("DATABASE_URL")
		.map_err(|e|format!("DATABASE_URL is not set in .env: {e}"))?;
	PgConnection::establish(&database_url)
		.map_err(|e| format!("Error connecting to {database_url}: {e}"))
}

