# Rust GraphQL Starter

This workspace contains the necesary crates to get a postgeSQL, graphQL backend server up and running. This is created using `async-graphl` and `sea-orm`.
The intent of this repository is to act as a template for starting Rust based graphQL projects.

This workspace currently has two crates:
* [**Migration**](/migration/README.md)
* [**Server**](/server/Readme.md)

## Getting Started

### Pre-requisites
* Install `Rust`. You can check it out [here](https://www.rust-lang.org/tools/install).

* A postreSQL database service up and running.

* Create an `.env` file from the `.sample.env`.

* A postreSQL database.

* Update the `.env` file with your database connection url.

  > For docker, you will want to create a `.docker.env` file. You can follow the `.sample.docker.env for help`

* Run the table migrations against your database with the following command in the terminal
  ```bash
  cargo run -p migration
  ```

### Running
You can run this application by using cargo or docker.

For cargo, you can use `cargo run -p server`.

For docker, you can use `docker compose up --build`.
>NOTE: Please make sure to create the `.docker.env` mentioned above

Your application will be available at http://localhost:4000.


## Deployment
This application can be deployed on any machine that can run a docker image. You can follow these steps to deploy this project:

* Create a `.env` & a `.docker.env` file from the `.sample.env` & `.sample.docker.env` files respectively.

  * Update the database url with a postgreSQL database url for both `.env` files. You can get one a free one in [Neon](https://neon.tech/).

  * Run migration with `cargo run -p migration`.

* Create an account in [docker.io](https://hub.docker.com/)

* Create a repository in your docker account.

  * Please note: If you create your repository as `private` you will have to create a `token` from your account settings page.



* Build the image using `docker build -t <docker-user-name>/<project-name> .`

  >If your cloud uses a different CPU architecture than your development machine (e.g: you are on a Mac M1 and your cloud provider is amd64), you'll want to build the image for that platform, e.g: `docker build --platform=linux/amd64 -t myapp .`

* Push the image into docker hub using `docker push <docker-user-name>/<project-name>`

  * Please make sure in docker hub that your image has been pushed.

* Provide the docker image link `<docker-user-name>/<project-name>` to your platform and it should get deployed.

  > As mentioned above, if your repository is `private` you will have to get an access `token`. In this step you will have to pass this `token` to the platform.