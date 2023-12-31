# Migration

SeaORM is a schema-first approach, so we have to run migrations against our database and then generate the entity files from our database.

This crate contains all the migrations that needs to be run against a database.

This is initialized using `sea-orm-cli migrate init`.

You can follow this [guide](https://www.sea-ql.org/sea-orm-tutorial/ch01-02-migration-cli.html#define-the-migrations) to create new migrations.

> You should NOT re-initialize this as it will remove all existing migrations.

## Running
To run the migrations you have to do the following:

* Create a `.env` file in the `root` crate. You can follow the `.sample.env` file for guidance, also please checkout the [Getting Started Guide](../Readme.md#getting-started).

* Run `cargo run -p migration`.
	> The migration crate accepts additional flags defined by sea-orm. You can check them out [here](https://www.sea-ql.org/SeaORM/docs/migration/running-migration/#command-line-interface-cli).
	> These commands can be used like so `cargo run -p migration -- COMMAND`

