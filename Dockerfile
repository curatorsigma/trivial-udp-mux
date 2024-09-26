FROM rust:1.80-alpine AS builder
RUN apk add --no-cache build-base
WORKDIR /usr/src/trivial-udp-mux
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /trivial-udp-mux
COPY --from=builder /usr/src/trivial-udp-mux/target/release/trivial-udp-mux ./
CMD ["./trivial-udp-mux"]

