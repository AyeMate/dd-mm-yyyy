FROM rust:1.83-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/dd-mm-yyyy .
EXPOSE 8080
CMD ["./dd-mm-yyyy"]
