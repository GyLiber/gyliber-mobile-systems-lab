FROM rust:1-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/* && useradd --system --uid 10001 lab
WORKDIR /app
COPY --from=builder /app/target/release/gyliber-mobile-systems-lab /usr/local/bin/lab
USER lab
ENV PORT=3000
EXPOSE 3000
CMD ["/usr/local/bin/lab"]
