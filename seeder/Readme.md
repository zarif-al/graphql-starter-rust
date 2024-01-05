# Seeder

This crate will seed your database with some dummy data. This should be updated when you add new entities to the project. It makes use of a rust crate called `fake` to generate fake data.

A key point to understand when creating a seed function for entities, is
that the `primary key` of a record must be defined in the seed data, even if its auto generated.

Since we use `transactions` to commit data into the database, we don't get the inserted data. But we need the `primary key` of seed data to make sure we can maintain relationships in all of our seeded data.

When this crate is run, it refreshes the database and then proceeds to seed data into it.

## Running

To run this crate make sure you have the `.env` file setup at the root crate.

Then you can run `cargo run -p seeder`.