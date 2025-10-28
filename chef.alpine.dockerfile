FROM blackdex/rust-musl:x86_64-musl-stable-1.89.0 AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

# Install sccache for compilation caching
RUN cargo install sccache --locked

# Configure sccache as the Rust compiler wrapper
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV SCCACHE_DIR=/sccache
ENV SCCACHE_CACHE_SIZE="10G"

# Configure OpenSSL for musl
ENV OPENSSL_DIR=/musl
ENV OPENSSL_STATIC=true

# create our plan
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build deps
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/sccache \
  cargo chef cook --release --target x86_64-unknown-linux-musl \
  --recipe-path recipe.json

# build our code
COPY . .
RUN --mount=type=cache,target=/sccache cargo build --release --target \
  x86_64-unknown-linux-musl --bin gha_demo

# runtime image
FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/gha_demo \
  /usr/local/bin/app

# just for convienence, but you should really consider using either docker
# volumes (comopose) or ConfigMaps (kubernetes) for deployments.
COPY configuration configuration

# need access to migrations when we start
COPY migrations migrations

USER myuser

CMD ["/usr/local/bin/app"]
