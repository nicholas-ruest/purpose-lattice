FROM rust:1.99-bookworm AS api
WORKDIR /app
COPY . .
RUN cargo build --locked --release -p pl-cli
FROM debian:bookworm-slim
COPY --from=api /app/target/release/purpose-lattice /usr/local/bin/purpose-lattice
ENV PL_BIND=0.0.0.0:8080
CMD ["purpose-lattice","serve"]
