# Migration

SeaORM is a schema-first approach, so we have to run migrations against our database and then generate the entity files from our database.

This crate contains all the migrations that needs to be run against a database.

> You will have to install `sea-orm-cli` with `cargo add sea-orm-cli`.

This is initialized using `sea-orm-cli migrate init`.

You can follow this [guide](https://www.sea-ql.org/sea-orm-tutorial/ch01-02-migration-cli.html#define-the-migrations) to create new migrations.

You can run the migrations using this command `sea-orm-cli migrate`.

> This is created using sea-orm docs.

> You should NOT re-initialize this as it will remove all existing migrations.

## Running
To run the migrations you have to do the following:

* Create a `.env` file in the `server` crate. You can follow the `Readme.md` in that crate to learn more.
* Run `cargo run -p migration`.

