# Migration

SeaORM is a schema-first approach, so we have to run migrations against our database and then generate the entity files from our database.

This crate contains all the migrations that needs to be run against a database.

This is initialized using `sea-orm-cli migrate init`.

> Since this crate is already initialized, you will not need to run this command again.

## Creating a new migration

To add new tables or make changes to existing table its best to create a new migration file. This will help you maintain an audit trail and allow you to easily rollback if something goes wrong.

Here are the instructions to creating a new migration:

- Use command `sea-orm-cli migrate generate new_table_name_here` to create a new migration.

	> You can pass a custom directory using the `-d` flag. If not passed it will look for the `migrations` folder in the path it is being run on.
- Apply your necessary actions in the `up` and `down` functions. Please take care to define these functions carefully. Specially when handling special columns of an already populated table.
- Double check the `migration/src/lib.rs` file. Make sure the vector in the `migrations` function is chronological in order. So your newly generated migration should be the last item in the vector.

## Running
To run the migrations you have to do the following:

* Create a `.env` file in the `root` crate. You can follow the `.sample.env` file for guidance, also please checkout the [Getting Started Guide](../Readme.md#getting-started).

* Run `cargo run -p migration`.
