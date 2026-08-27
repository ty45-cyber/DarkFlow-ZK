# Stage 1: Cargo Chef Planner
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
# Prepare a build plan for the workspace
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cacher
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build the actual application
COPY . .
# Build only the API binary from the workspace
RUN cargo build --release --package darkflow-api

# Stage 3: Runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install OpenSSL and CA certificates needed for Axum/HTTPS
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/darkflow-api /usr/local/bin/

# Railway injects the PORT env var automatically
EXPOSE ${PORT:-8080}

CMD ["darkflow-api"]