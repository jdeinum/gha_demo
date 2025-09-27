VERSION --global-cache 0.8
IMPORT github.com/earthly/lib/rust:3.0.1 AS rust

# install all of the tools we need, this should never need to change
install:
    FROM rust:1.87.0-slim-bookworm
    RUN rustup component add clippy rustfmt 
    RUN cargo install cargo-nextest
    RUN cargo install cargo-deny
    DO rust+INIT --keep_fingerprints=true

source:
  FROM +install
  COPY --keep-ts Cargo.toml Cargo.lock ./
  COPY --keep-ts --dir crates ./
  COPY --keep-ts --dir configuration ./
  COPY --keep-ts deny.toml ./


# build builds with the Cargo release profile
# TODO: Build with both targets
build:
  FROM +source
  ARG target='x86_64-unknown-linux-gnu'
  DO rust+CARGO --args="build --release" --output="release/[^/\.]+"
  RUN ls target/release/
  SAVE ARTIFACT target/release/producer artifacts/producer
  SAVE ARTIFACT target/release/consumer artifacts/consumer
  SAVE ARTIFACT ./configuration artifacts/configuration
  SAVE ARTIFACT deny.toml artifacts/deny.toml

# lint runs cargo clippy on the source code
lint:
  FROM +source
  DO rust+CARGO --args="clippy --all-features --all-targets -- -D warnings"

# test executes all unit and integration tests via Cargo
test:
  FROM +source
  DO rust+CARGO --args="nextest run --no-tests pass --release"

# fmt checks whether Rust code is formatted according to style guidelines
fmt:
  FROM +source
  DO rust+CARGO --args="fmt --check"

# audits our code, either on change to Cargo.toml or Cargo.lock, or on a timer
audit: 
  FROM +source 
  DO rust+CARGO --args="deny check advisories"

all:
  BUILD +build 
  BUILD +lint
  BUILD +test
  BUILD +fmt
