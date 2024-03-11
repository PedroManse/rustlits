use sqlx::sqlite::SqlitePool;

struct Account {
	id: i64,
	name: String,
}

struct Todo {
	id: i64,
	description: String,
	owner: i64,
}

#[tokio::main]
async fn main() {
	dotenvy::dotenv().expect("Couldn't read .env file");
	let url = std::env::var("DATABASE_URL").expect("Can't find DATABASE_URL in env");
	let pool = SqlitePool::connect(&url).await.expect("Can't connect to sqlite3 db");

	let accs = get_accounts(&pool).await;
	if accs.is_empty() {
		println!("added user! id: {}", add_account(&pool, "Manse").await);
	}
	let accs = get_accounts(&pool).await;

	for acc in accs.into_iter() {
		let todos = acc.get_todos(&pool).await;
		if todos.is_empty() {
			acc.add_todo(&pool, "TODO!!").await;
		}
		let todos = acc.get_todos(&pool).await;

		println!("acc #{} \"{}\"", acc.id, acc.name);
		for todo in todos.into_iter() {
			println!("    #{}/{} \"{}\"", todo.owner, todo.id, todo.description);
		}
		println!();
	}
}

async fn get_accounts(pool: &SqlitePool) -> Vec<Account> {
	sqlx::query_as!(
		Account,
		"SELECT id, name FROM accounts",
	).fetch_all(pool).await.expect("can't select accounts")
}

impl Account {
	async fn get_todos(&self, pool: &SqlitePool) -> Vec<Todo> {
		sqlx::query_as!(
			Todo, r#"
	SELECT
		id, description, owner
	FROM
		todos
	WHERE
		id=?
	"#, self.id)
			.fetch_all(pool).await.expect("can't select todos")
	}
	async fn add_todo(&self, pool: &SqlitePool, description: &str) -> i64 {
		sqlx::query!(r#"
	INSERT INTO todos
		(description, owner)
	VALUES
		(?, ?)
	"#, description, self.id)
			.execute(pool)
			.await
			.expect("RUNTIME sqlite error")
			.last_insert_rowid()
	}
}

async fn add_account(pool: &SqlitePool, name: &str) -> i64 {
	sqlx::query!(r#"
INSERT INTO accounts
	(name)
VALUES
	(?)
"#, name)
		.execute(pool)
		.await
		.expect("RUNTIME sqlite error")
		.last_insert_rowid()
}

