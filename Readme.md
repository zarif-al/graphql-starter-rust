# Rust GraphQL Starter

This repository should be used as a template for starting Rust based graphQL server project.

## Crates
This template makes use of the following crates:
- [Axum](https://docs.rs/axum/latest/axum/)
- [Async-GraphQL](https://async-graphql.github.io/async-graphql/en/introduction.html)
- [Tracing](https://crates.io/crates/tracing)

#### Notes
- We cannot use the latest version of `axum` (0.7) as `async-graphql` does not yet support it ([PR-1431](https://github.com/async-graphql/async-graphql/pull/1431)).

## Folder Structure

In this project we should try to encapsulate all unique logic in its own modules/library files. Please go through the existing modules.

> All modules should be documented.

## Logging

Logging in this project is done with the `tracing` crate. It currently only configured to print to the stadard output.

## Building and running your application
You can run this application by using cargo or docker. For cargo you can use standard cargo commands to build and run the project.

For docker you can use `docker compose up --build`.

Your application will be available at http://localhost:4000.

## Deployment
This application can be deployed on any machine that can run a docker image. You can follow these steps to deploy this project:

- Create an account in [docker.io](https://hub.docker.com/)
- Create a repository in your docker account.
  - Please note: If you create your repository as `private` you will have to create a `token` from your account settings page.
- Build the image using `docker build -t <docker-user-name>/<project-name> .`
  >If your cloud uses a different CPU architecture than your development machine (e.g: you are on a Mac M1 and your cloud provider is amd64), you'll want to build the image for that platform, e.g: `docker build --platform=linux/amd64 -t myapp .`
- Push the image into docker hub using `docker push <docker-user-name>/<project-name>`
  - Please make sure in docker hub that your image has been pushed.
- Provide the docker image link `<docker-user-name>/<project-name>` to your platform and it should get deployed.
  > As mentioned above, if your repository is `private` you will have to get an access `token`. In this step you will have to pass this `token` to the platform.