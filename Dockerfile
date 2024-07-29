FROM rust as build

RUN USER=root cargo new --bin cantellation
WORKDIR /cantellation

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/cantellation*
RUN cargo build --release

FROM debian:12-slim

COPY --from=build /cantellation/target/release/cantellation .

CMD ["./cantellation"]
