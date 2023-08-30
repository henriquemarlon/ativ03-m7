# Build stage
FROM rust:latest as builder

WORKDIR /app

# accept the build argument
COPY . . 

RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/notex .

CMD ["./rust-crud-api"]