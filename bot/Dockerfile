FROM rust:1.66-slim-buster AS builder
WORKDIR /builder

COPY Cargo.toml Cargo.lock ./
COPY bot/Cargo.toml bot/Cargo.toml
RUN mkdir bot/src && \
    touch bot/src/lib.rs && \
    cargo build --release --package bot && \
    rm -r bot/src

COPY bot/src bot/src
RUN cargo build --release --package bot

FROM debian:buster-slim AS runner
WORKDIR /runner

COPY --from=builder /builder/target/release/bot /runner/bin
COPY bot/config.toml bot/config.toml
ENTRYPOINT ["./bin"]
