FROM messense/rust-musl-cross:x86_64-musl as chef
RUN cargo install cargo-chef
WORKDIR /app

RUN apt-get update && apt install -y openssl && apt install -y ca-certificates

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# We do not need the Rust toolchain to run the binary!
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/builder_connect /app
EXPOSE 8080
ENTRYPOINT ["/app"]