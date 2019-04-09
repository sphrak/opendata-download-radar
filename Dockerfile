FROM rustlang/rust:nightly as builder

ARG ARCH=x86_64-unknown-linux-musl

# Build image
WORKDIR /app/src
# ADD Cargo.toml Cargo.lock ./
# RUN cargo fetch --locked -v
ADD ./ ./
RUN rustup target add "${ARCH}" \
	&& cargo build --release \
		--package opendata-download-radar \
		--target "${ARCH}" \
		-vv

# Runtime image
FROM alpine:3.9
WORKDIR /app
COPY --from=builder /app/src/target/${ARCH}/release/opendata-download-radar .
CMD ["opendata-download-radar"]