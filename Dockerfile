FROM rust:latest as builder
WORKDIR /app
COPY src/main.rs src/main.rs
RUN rustc -C target-cpu=native -C opt-level=3 -C lto=fat -C panic=abort -C codegen-units=1 -C strip=symbols src/main.rs -o atcs

FROM debian:latest
WORKDIR /app
COPY --from=builder /app/atcs /app/atcs
EXPOSE 8082
ENTRYPOINT ["/app/atcs"]
