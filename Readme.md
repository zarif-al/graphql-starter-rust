# Rust GraphQL Starter

This repository should be used as a template for starting Rust based graphQL server project.

## Crates
This template makes use of the following crates:
- Axum
- Async-GraphQL
- Tracing

### Notes
- We cannot use the latest version of axum (0.7) as async-graphql does not yet support it.

## Folder Structure

We should try to isolate any unique logic into its own module/library. In this starter code we have the following modules:

- graphql
  - query
  - mutation
- general_response
- health_check
- user
  - definition
  - create
- env

All modules should be documented.