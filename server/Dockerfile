# Build Stage
FROM rust:1.75-buster as builder

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY ./server /app

RUN cargo build --release

# Production Stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/zibelina_store .

CMD [ "./zibelina_store" ]