use crate::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: uuid::Uuid,
    pub cnpj: String,
    pub name: String,
    pub passhash: Vec<u8>,
    pub isadmin: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::accounts)]
pub struct NewAccount<'a> {
    pub cnpj: &'a str,
    pub name: &'a str,
    pub passhash: Vec<u8>,
}

#[derive(Debug)]
pub enum NewAccountError {
    Diesel(diesel::result::Error),
    InvalidCNPJ(String),
    WrongFormatCNPJ,
    Password(argon2::Error),
}

use std::convert::From;
impl From<NewAccountError> for String {
    fn from(item: NewAccountError) -> String {
        use NewAccountError::*;

        match item {
            InvalidCNPJ(st) => format!("Invalid CNPJ: {st}"),
            WrongFormatCNPJ => "CNPJ must be 14 digits".to_owned(),
            Password(reason) => format!("Error hashing password: {reason}"),
            Diesel(e) => format!("DB diesel error: {e}"),
        }
    }
}

//TODO: validate cnpj
impl<'a> NewAccount<'a> {
    pub fn new(cnpj: &'a str, name: &'a str, pass: &'a str) -> Result<Self, NewAccountError> {
        if cnpj.len() != 14 || !cnpj.chars().all(char::is_numeric) {
            Err(NewAccountError::WrongFormatCNPJ)
        } else {
            let passhash = diesel_example::hash_password(pass.as_bytes())
                .map_err(NewAccountError::Password)?;
            Ok(NewAccount {
                cnpj,
                name,
                passhash,
            })
        }
    }

    pub fn save(self, con: &mut PgConnection) -> Result<Account, NewAccountError> {
        diesel::insert_into(schema::accounts::table)
            .values(&self)
            .returning(Account::as_returning())
            .get_result(con)
            .map_err(NewAccountError::Diesel)
    }
}

impl Account {
    pub fn get_all(con: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::accounts::dsl::*;
        accounts.select(Account::as_select()).load(con)
    }

    pub fn from_uuid(con: &mut PgConnection, uid: uuid::Uuid) -> Option<Self> {
        use crate::schema::accounts::dsl::*;
        accounts.find(uid).first(con).ok()
    }

    pub fn from_cnpj(con: &mut PgConnection, cnpj: &str) -> Option<Self> {
        use crate::schema::accounts::dsl;
        dsl::accounts.filter(dsl::cnpj.eq(cnpj)).first(con).ok()
    }
}
