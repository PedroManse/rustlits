// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        #[max_length = 14]
        cnpj -> Bpchar,
        name -> Text,
        passhash -> Bytea,
        isadmin -> Bool,
    }
}
