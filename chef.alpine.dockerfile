FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

# create our plan
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build deps
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# build our code
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin app

# runtime image
FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/app

# just for convienence, but you should really consider using either docker
# volumes (comopose) or ConfigMaps (kubernetes) for deployments.
COPY configuration configuration

# need access to migrations when we start
COPY migrations migrations

USER myuser

CMD ["/usr/local/bin/app"]
