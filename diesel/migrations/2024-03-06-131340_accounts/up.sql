-- Your SQL goes here
CREATE TABLE IF NOT EXISTS accounts (
	id       UUID     NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	cnpj     CHAR(14) NOT NULL UNIQUE,
	name     TEXT     NOT NULL,
	passhash BYTEA    NOT NULL,
	isAdmin  BOOLEAN  NOT NULL DEFAULT false
);

