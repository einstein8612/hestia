FROM rust:latest

COPY . .

# Build hestia
RUN cargo build --release
RUN mv /target/release/server /

# Run the binary
CMD ["./server"]
