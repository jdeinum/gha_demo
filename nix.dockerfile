# Multi-stage build using Nix
FROM nixos/nix:latest AS builder

# Enable experimental features for flakes
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Copy our source and setup our working dir
COPY . /tmp/build
WORKDIR /tmp/build

# Build our Nix environment
RUN nix \
  --extra-experimental-features "nix-command flakes" \
  --option filter-syscalls false \
  build

# Copy the Nix store closure into a directory. The Nix store closure is the
# entire set of Nix store values that we need for our build.
RUN mkdir /tmp/nix-store-closure
RUN cp -R $(nix-store -qR result/) /tmp/nix-store-closure

# Final image is based on scratch. We copy a bunch of Nix dependencies
# but they're fully self-contained so we don't need Nix anymore.
FROM scratch

WORKDIR /app

# Copy /nix/store
COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/build/result /app
COPY --from=builder /tmp/build/configuration /app/configuration
COPY --from=builder /tmp/build/migrations /app/migrations

# Expose the port the app runs on
EXPOSE 8080

# Set environment variables
ENV APP_ENV=production
ENV RUST_LOG=info

# Run the binary
CMD ["/app/bin/app"]
