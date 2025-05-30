# ---------- Stage 1: Builder ----------
FROM rust:1.80.0 AS builder

# Create a new app dir inside the container
WORKDIR /product-service

# Copy just Cargo.toml and Cargo.lock first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Pre-create a dummy main.rs to allow `cargo build` to work
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this step will be cached unless Cargo.toml/lock changes)
RUN cargo build --release && rm -rf src

# Now copy the real source code
COPY . .

# Build the actual project
RUN cargo build --release

# ---------- Stage 2: Runner ----------
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Set the build argument for the app version number
ARG APP_VERSION=0.1.0

# Install runtime dependencies (e.g., for OpenSSL)
RUN apt-get update && apt-get install -y wget libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy only the compiled binary from the builder
COPY --from=builder /product-service/target/release/product-service .

# Set environment variables
ENV APP_VERSION=$APP_VERSION

# Run the binary
CMD ["./product-service"]
