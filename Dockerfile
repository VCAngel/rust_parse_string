FROM rust:1.84-slim as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/parse_string /usr/local/bin/parse_string

CMD ["parse_string"]
