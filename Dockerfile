FROM rust:latest as builder 
MAINTAINER Niclas Kron <niclas.kroon@protonmail.ch>

ARG ARCH=x86_64-unknown-linux-musl
ARG RUST_VER=nightly-2019-04-07

# Build image
WORKDIR /app/src
ADD Cargo.toml Cargo.lock ./
RUN cargo fetch --locked -v
ADD ./ ./
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --package opendata-download-radar --target "${ARCH}" -v --frozen

# Runtime image
FROM alpine:3.9
ADD ./src /app
WORKDIR /app
COPY --from=builder /app/src/target/${ARCH}/release/opendata-download-radar .
CMD ["opendata-download-radar"]
