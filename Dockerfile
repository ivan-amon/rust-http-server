# ---- Build stage ----
FROM rust:1-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ---- Runtime stage ----
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/rust-http-server ./
COPY hello.html 404.html ./

EXPOSE 80

CMD ["./rust-http-server"]
