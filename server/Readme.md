<!-- omit in toc -->
# Server
This crate makes use of the following dependency crates:
- [Axum](https://docs.rs/axum/latest/axum/): Build and run the server.
- [Async-GraphQL](https://async-graphql.github.io/async-graphql/en/introduction.html): GraphQL library.
- [Tracing](https://crates.io/crates/tracing): Logging Solution.
- [SeaORM](https://www.sea-ql.org/SeaORM/docs/index/): Database ORM.

<!-- omit in toc -->
# Table of Contents
- [Modules](#modules)
	- [GraphQL](#graphql)
		- [Repositories](#repositories)
- [Testing](#testing)
- [Error Codes](#error-codes)


## Modules

### GraphQL
This module contains the `Query`, `Mutation` and `Subscriptions` structs and their method implementations.

The methods defined in the above structs are used by `async-graphql` to create the graphQL server.

#### Repositories
This a submodule of graphql and is designed to be similar to a concept from NestJS.

This module contains submodules for each `entity` defined in the `entities` module. For example the `User` entity will have a corresponding module in `Respositories`.

These submodules will contain CRUD operations for that entity.

Some of the datatypes used for an entities properties are not serializable. Therefore they cannot be used by `async-graphql`.

There are quite a few different solutions, for instance we can update the `entities` folder to contain wrapper types for the non-serializable types.

The drawback of this approach is the `entities` file gets overwritten when we generate the entities module. This is a likely scenario as our tables will get updated as a project proceeds.

My solution is to have a GraphQL stuct for every entity struct. The GraphQL struct can have all the properties of the entity struct and use serializable types for all properties.

> We also have the flexibility of inlcuding extra properties or omitting specific properties if we don't need it. We can also alter the datatype of a property.

The GraphQL struct will also need to implement a `From trait` to convert the entity struct model to the GraphQL struct.

> Please note the from trait is specified using the import path `crate::entity::...`. If you import an entity using `server::entity::...` then the `From trait` will not work.

We call the structs `GraphQL<entity-name>`, i.e `GraphQLUser`.

You can checkout the [user](/src/grapqhl/repositories/user/mod.rs) module in repositories to learn more.

## Testing

I have opted to use the Mock database approach for testing. You can learn more about this from sea-orms [docs](https://www.sea-ql.org/SeaORM/docs/write-test/mock/).

> This methodology **does not** mimic a real world database, the queries are not carried out as they would have been in a real database. Instead the mock database will return the mock data that **you define**. We are **not testing** the database.

This methodology does let us make sure that our functions are processing the data from the database and returning them correctly.

## Error Codes

I think its a good idea to return custom error codes with specific meaning rather than returning error message strings.

It makes it easier to handle the error and display appropriate (even multilingual) messages in the frontend.


* **500**: `Internal server error`
* **I100**: `Invalid Input`
* **U100**: `User not found`
* **P100**: `Post not found`
* **P101**: `Post does not belong to user`
