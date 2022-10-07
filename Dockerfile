FROM rust:1.62
COPY . .
RUN cargo build --release
CMD ["./target/release/ipfs-api"]