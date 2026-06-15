# Stage 1: Build the application
FROM rust:1.95.0-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-binstall to speed up tool installation
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos and add WASM target
RUN cargo binstall -y cargo-leptos
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

# Build the project in release mode
RUN cargo leptos build --release

# Stage 2: Runner
FROM debian:bookworm-slim AS runner

# Install runtime dependencies (like libssl, ca-certificates)
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary and site assets from the builder stage
COPY --from=builder /app/target/release/open-diy /app/
COPY --from=builder /app/target/release/hash.txt /app/
COPY --from=builder /app/target/site /app/site


# Set environment variables for Leptos SSR
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_ENV=PROD
ENV LEPTOS_HASH_FILES=true


# Expose port
EXPOSE 3000

# Run the binary
CMD ["/app/open-diy"]
