FROM messense/rust-musl-cross:x86_64-musl as chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN ls -la /app
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
RUN apt-get update && apt install -y openssl && apt install -y ca-certificates
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final stage: the scratch image
FROM scratch
# Copy the binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/builder_connect /app
# Copy the CA certificates from the builder stage
COPY --from=builder /etc/ssl/certs /etc/ssl/certs
# Set the environment variable so your application knows where to find the CA certificates
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
EXPOSE 8080
ENTRYPOINT ["/app"]
