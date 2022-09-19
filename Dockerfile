FROM rust:latest as builder
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder ./target/release/serenity-arma ./target/release/serenity-arma
CMD ["/target/release/serenity-arma"]