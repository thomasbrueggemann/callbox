FROM rust:1.56 as build

RUN USER=root cargo new --bin callbox
WORKDIR /callbox

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim
RUN apt update
RUN apt install openssl apt-transport-https ca-certificates -y

COPY --from=build /callbox/target/release/callbox .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./callbox"]