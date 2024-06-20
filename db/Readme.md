# DB

We are using `SeaORM` as our tool to communicate with the database. This orm supports the following databases:
- MySQL
- PostgreSQL
- SQLite

The project is setup to work with `PostgreSQL` database but you can adjust it for any of the other options with minimal change to the code.

> This library crate contains and exports the definitions for `ENV` struct and the `db_connection` funciton.