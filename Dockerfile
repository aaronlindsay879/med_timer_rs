FROM rustlang/rust:nightly-slim as builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder target/release/med_timer_server /usr/local/bin/med_timer
COPY testing.db .

CMD ["med_timer"]