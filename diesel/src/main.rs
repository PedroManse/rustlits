pub mod schema;
pub mod models;
use models::{NewAccount, Account};
use diesel_example::*;

fn main() -> Result<(), String> {
	let con = &mut establish_connection()?;

	let acc = NewAccount::new("55587095000003", "Manse 3", "@BakedInPass")?.save(con);
	println!("new account: {acc:?}");

	println!("{:?}", Account::from_cnpj(con, "55587095000003").unwrap());
	for acc in Account::get_all(con).unwrap() {
		println!("{acc:?}");
	}
	Ok(())
}

