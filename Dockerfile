FROM rust:1.90-slim-bookworm AS builder

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /silf

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/Silf ./silf

EXPOSE 8080

CMD ["./silf"]
