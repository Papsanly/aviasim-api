FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /aviasim-orders
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /aviasim-orders/target/x86_64-unknown-linux-musl/release/aviasim-orders /aviasim-orders
ENTRYPOINT ["/aviasim-orders"]
EXPOSE 3472
