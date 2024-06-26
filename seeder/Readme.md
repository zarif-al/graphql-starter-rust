# Seeder

This crate will seed your database with some dummy data. This should be updated when you add new entities to the project. It makes use of a rust crate called `fake` to generate fake data.

You can seed data within the `up` function of a migration. But I wanted to keep these two functions separate. I only want to seed in my development/staging database, not in the production database. If the seed function is part of the `up` migration then production database would also end up with seed data.

A key point to understand when creating a seed function for entities, is
that the `primary key` of a record must be defined in the seed data, even if its auto generated.

Since we use `transactions` to commit data into the database, we don't get the inserted data. But we need the `primary key` of seed data to make sure we can maintain relationships in all of our seeded data.

When this crate is run, it refreshes the database and then proceeds to seed data into it.

## Creating New Seed Function

Creating a new seed function for a new entity is as simple as creating a new module with the necessary `generate` and `seed` functions.

Then you can add the seed function to the `main.rs` file.

> Make sure to maintain the sequence of entities being synced if the new entity has relationships to otehr tables.

## Running

To run this crate make sure you have the `.env` file setup at the root crate.

Then you can run `cargo run -p seeder`.