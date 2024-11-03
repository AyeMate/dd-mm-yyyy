FROM rust:1.82-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM scratch
WORKDIR /app
COPY --from=builder /app/target/release/dd-mm-yyyy .
EXPOSE 8080
CMD ["./dd-mm-yyyy"]
