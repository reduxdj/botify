FROM rust:bookworm as builder
RUN USER=root cargo new --bin botify_server
WORKDIR /botify_server
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm -f ./target/release/deps/botify_server*
RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && apt install -y openssl
COPY --from=builder /botify_server/target/release/botify_server /usr/local/bin/botify_server
EXPOSE 4000
CMD ["botify_server"]
