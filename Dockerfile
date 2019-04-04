FROM rust:1.31
ADD ./src /app
WORKDIR /app
RUN cargo install --path .
# copy over the binary
CMD ["opendata-download-radar"]
