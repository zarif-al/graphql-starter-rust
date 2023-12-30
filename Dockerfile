# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.74.0

################################################################################
# xx is a helper for cross-compilation.
# See https://github.com/tonistiigi/xx/ for more information.
FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx

################################################################################
# Create a stage for building the application.
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS builder

# Set the working directory inside the container
WORKDIR /usr/src/server

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git file

# Copy the entire project to the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Set the startup command for the container
# CMD ["./target/release/server"]

# Create a new lightweight image
FROM alpine:3.18

# Set the working directory inside the container
WORKDIR /usr/src/server

# Copy only the necessary files from the builder stage
COPY --from=builder /usr/src/server/target/release/server .

# Copy the .docker.env file
COPY .docker.env /.env

# Expose the port your application will run on (if needed)
EXPOSE 4000

# Set the startup command for the container
CMD ["./server"]
