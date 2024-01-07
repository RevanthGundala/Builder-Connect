FROM lukemathwalker/cargo-chef:0.1.55-rust-slim-bullseye AS chef
WORKDIR /app

RUN apt-get update && apt install -y openssl && apt install -y ca-certificates

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin builder_connect

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/builder_connect /usr/local/bin
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/builder_connect"]