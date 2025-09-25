FROM rust:1.83.0-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app

# копіюємо код
COPY . .

# додаємо musl target
RUN rustup target add x86_64-unknown-linux-musl

# білдимо під musl (статично)
RUN cargo build --release --target x86_64-unknown-linux-musl