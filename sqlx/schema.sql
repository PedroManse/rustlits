CREATE TABLE IF NOT EXISTS accounts (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS todos (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	description TEXT NOT NULL,
	owner INTEGER NOT NULL,
	FOREIGN KEY(owner) REFERENCES accounts(id)
);