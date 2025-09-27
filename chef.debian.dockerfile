# Define a build-time variable for the binary name
ARG BINARY_NAME=api

# Use Rust image for building
FROM lukemathwalker/cargo-chef:latest-rust-1.89.0 AS chef
WORKDIR /app
RUN apt update

# create our recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build deps
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# build our code
COPY . .
RUN cargo build --release --bin

# final runtime image
FROM debian:bookworm-slim AS runtime
WORKDIR /app
ENV APP_ENV=production
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/app app

# just for convienence, but you should really consider using either docker
# volumes (comopose) or ConfigMaps (kubernetes) for deployments.
COPY configuration configuration

# need access to migrations when we start
COPY migrations migrations

# run
ENTRYPOINT ["./app"]

