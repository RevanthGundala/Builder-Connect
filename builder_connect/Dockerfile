FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /builder_connect

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /builder_connect/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin builder_connect

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /builder_connect
COPY --from=builder /builder_connect/target/release/builder_connect /usr/local/bin
EXPOSE 8080
RUN apt-get update && apt install -y openssl && apt install -y ca-certificates

ENTRYPOINT ["/usr/local/bin/builder_connect"]