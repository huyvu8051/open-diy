# Stage 1a: shared toolchain (chef + cargo-leptos + wasm target) for the
# planner and builder stages below. cargo-chef splits the build into a
# dependency-only layer (cached across builds as long as Cargo.toml/
# Cargo.lock don't change) and an app-code layer (rebuilds every time) —
# previously every push recompiled all ~180 dependency crates from
# scratch (~12-13 min), since this Dockerfile just did `COPY . .` +
# `cargo leptos build` with no caching at all.
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall -y cargo-leptos
RUN rustup target add wasm32-unknown-unknown

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# `cargo leptos build --release` actually runs two separate `cargo build`
# invocations under the hood (confirmed via its own build output):
#   cargo build --bin=open-diy --no-default-features --features=ssr,trailing_telemetry
#   cargo build --lib --target=wasm32-unknown-unknown --target-dir=target/front \
#     --no-default-features --features=hydrate  (profile "wasm-release", see Cargo.toml)
# Cooking dependencies for both, with matching flags/target-dir, so the
# real build below finds a warm cache for both instead of only the native
# side (or a wasm cache sitting in the wrong directory it never looks in).
RUN cargo chef cook --release --no-default-features --features ssr,trailing_telemetry --recipe-path recipe.json
RUN cargo chef cook --profile wasm-release --target wasm32-unknown-unknown --target-dir target/front --no-default-features --features hydrate --recipe-path recipe.json

# Build the project in release mode
COPY . .
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
COPY --from=builder /app/target/site /app/site
# <HydrationScripts>/<HashedStylesheet> look for this file next to the
# running binary (current_exe().parent()) to know the content hashes
# cargo-leptos already baked into the js/wasm/css filenames under
# target/site/pkg — without it they silently fall back to unhashed names
# that don't exist on disk anymore. See the hash-files note in Cargo.toml.
COPY --from=builder /app/target/release/hash.txt /app/


# Set environment variables for Leptos SSR
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_ENV=PROD
ENV LEPTOS_HASH_FILES=true



# Expose port
EXPOSE 3000

# Run the binary
CMD ["/app/open-diy"]
