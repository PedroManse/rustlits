use maud::{DOCTYPE, html, Markup, Render, PreEscaped};

const HTMX:PreEscaped<&str> = PreEscaped(r#"<script src="https://unpkg.com/htmx.org@1.9.10"></script>"#);

struct CSS(&'static str);
impl Render for CSS {
	fn render(&self) -> Markup {
		html! { link rel="stylesheet" type="text/css" href=(self.0); }
	}
}

fn header(title: &str) -> Markup {
	html! { (DOCTYPE) meta charset="utf-8"; title { (title) } }
}

use axum::debug_handler;
#[debug_handler]
pub async fn index() -> Markup {
	html! {html lang="pt-br" {
		(header("Index"))

		head { (CSS("/files/css/index.css")) }


		body {
			h1 { "Hello!" }
		}

	}}
}

use crate::Account;
impl Render for Account {
	fn render(&self) -> Markup {
		html!{
			tr {
				th { (self.cnpj) }
				td { (self.name) }
				td { (self.isadmin) }
			}
		}
	}
}

use crate::{State, StatusCode};
pub async fn list_users(
	State(con): State<deadpool_diesel::postgres::Pool>,
) -> Result<Markup, StatusCode> {
	let accounts = Account::get_all(con).await.unwrap();

	Ok( html!{
		head { (CSS("/files/css/index.css")) }

		table {
			thead {
				tr {
					td{ "CNPJ" }
					td{ "nome" }
					td{ "é admin" }
				}
			}
			tbody {
				@for account in accounts {
					(account.render())
				}
			}
		}
	} )
}
