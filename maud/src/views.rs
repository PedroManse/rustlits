use maud::{DOCTYPE, html, Markup, Render, PreEscaped};

const HTMX:PreEscaped<&str> = PreEscaped(r#"<script src="https://unpkg.com/htmx.org@1.9.10"></script>"#);

struct SCRIPT(&'static str);
impl Render for SCRIPT {
	fn render(&self) -> Markup {
		html! { script src=(self.0); }
	}
}

struct CSS(&'static str);
impl Render for CSS {
	fn render(&self) -> Markup {
		html! { link rel="stylesheet" type="text/css" href=(self.0); }
	}
}

fn header(title: &str) -> Markup {
	html! { (DOCTYPE) meta charset="utf-8"; title { (title) } }
}

pub fn index() -> Markup {
	html! {html lang="pt-br" {
		head {
			(header("Index")) // execute (may change)
			(CSS("/files/css/index.css")) // dependes on which page you are rendering
			(HTMX) // 'pre' executed (all code uses this one piece)
		}

		body {
			h1 { "Hello!" }
		}

	}}
}

struct Account {
	cnpj: String,
	name: String,
	is_admin: bool,
}
impl Account {
	fn new(cnpj: String, name: String) -> Self {
		Account{cnpj, name, is_admin: false}
	}
	fn adm(cnpj: String, name: String) -> Self {
		Account{cnpj, name, is_admin: true}
	}
}
impl Render for Account {
	fn render(&self) -> Markup {
		html!{
			tr {
				th { (self.cnpj) }
				td { (self.name) }
				td { (self.is_admin) }
			}
		}
	}
}

pub fn list_users() -> Markup {
	let accounts = vec![
		Account::new("40481093841830".to_owned(),   "Jon A.".to_owned()),
		Account::new("40481093841831".to_owned(),  "Carl B.".to_owned()),
		Account::new("40481093841832".to_owned(),  "Josh C.".to_owned()),
		Account::adm("40481093841833".to_owned(), "Clara H.".to_owned()),
	];
	html!{
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
					(account)
				}
			}
		}
	}
}


