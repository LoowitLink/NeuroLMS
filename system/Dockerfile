# Use Rust official image for building
FROM rust:1.79.0
COPY . .

RUN cargo build --release


EXPOSE 8080
CMD ["./target/release/scholaralms_system"]