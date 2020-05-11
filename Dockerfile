FROM rust:1.43.1 as builder

WORKDIR /ajisen
COPY Cargo.toml Cargo.lock ./

# Initial build to cache dependencies
RUN mkdir -p ./src \
  &&  echo 'fn main() { println!("Ajisen") }' > ./src/main.rs \
  && cargo build --release --bin ajisen \
  && rm -r ./target/release/.fingerprint/ajisen-*

COPY src src

RUN cargo build --frozen --release --bin ajisen

FROM debian:stable-slim

WORKDIR /ajisen
COPY config config
COPY --from=builder /ajisen/target/release/ajisen .
ENTRYPOINT ["./ajisen"]
