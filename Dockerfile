FROM rust:1.75 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build -p kexa-node --release

FROM debian:bookworm-slim
RUN useradd -m kexa
WORKDIR /app
COPY --from=builder /app/target/release/kexa-node /usr/local/bin/kexa-node
USER kexa
ENTRYPOINT ["/usr/local/bin/kexa-node"]
