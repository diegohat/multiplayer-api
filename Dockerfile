FROM rust:alpine3.20 AS builder
RUN apk add --no-cache cmake build-base pkgconfig openssl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.20
RUN apk add --no-cache libssl3
WORKDIR /app
COPY --from=builder /app/target/release/api /app/api
COPY .env .env
RUN chmod +x /app/api
EXPOSE 3000
ENV RUST_LOG=debug
CMD ["/app/api"]