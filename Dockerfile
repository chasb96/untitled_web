FROM rust AS build_host
WORKDIR /src

RUN USER=root cargo new --bin web_bff
WORKDIR /src/web_bff

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/web_bff*

COPY ./src ./src
RUN cargo build --release

WORKDIR /src

FROM rust:slim

WORKDIR /src

COPY --from=build_host /src/web_bff/target/release/web_bff ./web_bff

CMD ["./web_bff"]
