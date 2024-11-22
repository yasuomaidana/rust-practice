FROM rust:1.82.0 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/app/target/release/distrosless-tokenizer /usr/local/bin/distrosless-tokenizer
CMD ["distrosless-tokenizer"]
