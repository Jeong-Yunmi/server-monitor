# Builder stage
FROM rust:1.75 as builder

WORKDIR /usr/src/flick
RUN cargo new --bin server
WORKDIR /usr/src/flick/server

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies only
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY src ./src

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/flick/server/target/release/flick_server /usr/local/bin/flick_server

# Set user for security
RUN useradd -m flick
USER flick

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["flick_server"]