FROM rust:1.88.0-bullseye AS builder

WORKDIR /app
COPY . .

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu --package plugind

FROM debian:bullseye

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/plugind /usr/local/bin/plugind

CMD ["/usr/local/bin/plugind"]