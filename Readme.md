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

# Deployment
This application can be deployed on any machine that can run a docker image. You can follow these steps to deploy this project:

- Create an account in [docker.io](https://hub.docker.com/)
- Create a repository in your docker account.
  - Please note: If you create your repository as `private` you will have to create a `token` from your account settings page.
- Build the image using `docker build -t <docker-user-name>/<project-name>`
- Push the image into docker hub using `docker push <docker-user-name>/<project-name>`
  - Please make sure in docker hub that your image has been pushed.
- Provide the docker image link `<docker-user-name>/<project-name>` to your platform and it should get deployed.
  > As mentioned above, if your repository is `private` you will have to get an access `token`. In this step you will have to pass this `token` to the platform.