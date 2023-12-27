# Rust GraphQL Starter

This is a postgreSQL, graphql backend crate, created using `async-graphql` and `sea-orm`. This is created to be used a template for starting Rust based graphQL backend projects.

If you want to learn more about the project's folder/code structure you can checkout [this readme](/Project-Details.md).

## Getting Started

### Pre-requisites
- A postreSQL database service up and running.
- A database.
- Create an `.env` file from the `.sample.env`.
- Update the `.env` file with your database connection url.
- For docker, you will want to create a `.docker.env` file.
- Install `sea-orm-cli` with `cargo install sea-orm-cli`
- Run the table migrations against your database with the following command in the terminal
  ```bash
  sea-orm-cli migrate
  ```
### Running
You can run this application by using cargo or docker. For cargo you can use standard cargo commands to build and run the project.

For docker, you can use `docker compose up --build`.
>NOTE: Please make sure to create the `.docker.env` mentioned above

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