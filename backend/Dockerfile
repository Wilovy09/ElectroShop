FROM rust:alpine3.19 AS builder

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM alpine:3.19

RUN apk add --no-cache ca-certificates

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/ .

EXPOSE 8080

CMD ["./target/release/backend"]
