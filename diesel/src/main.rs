pub mod schema;
pub mod models;
use models::{NewAccount, Account};
use diesel_example::*;


impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Acc")
           .field("Is Admin", &self.isadmin)
           .field("Name", &self.name)
           .field("CNPJ", &self.cnpj)
           .field("ID", &self.id)
           .finish()
    }
}

fn main() -> Result<(), String> {
	let con = &mut establish_connection()?;

  if let Some(acc) = Account::from_cnpj(con, "55587095000003") {
      println!("account already exists: {acc:?}");
  } else {
      let acc = NewAccount::new("55587095000003", "Manse 3", "@BakedInPass")?.save(con)?;
      println!("new account: {acc:?}");
  }

	println!("{:?}", Account::from_cnpj(con, "55587095000003").unwrap());
	for acc in Account::get_all(con).unwrap() {
		println!("{acc:?}");
	}
	Ok(())
}

