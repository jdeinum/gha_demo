# Define a build-time variable for the binary name
ARG BINARY_NAME=api

# Use Rust image for building
FROM lukemathwalker/cargo-chef:latest-rust-1.85.1 as chef
WORKDIR /app
RUN apt update

# create our recipe
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build deps
FROM chef as builder
ARG BINARY_NAME
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# build our code
COPY . .
RUN cargo build --release --bin ${BINARY_NAME}

# final runtime image
FROM debian:bookworm-slim AS runtime
WORKDIR /app
ARG BINARY_NAME
ENV APP_ENV=production
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/${BINARY_NAME} ${BINARY_NAME}
COPY configuration configuration
COPY migrations migrations

# run
ENTRYPOINT ["./${BINARY_NAME}"]

