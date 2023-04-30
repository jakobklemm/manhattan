FROM rust:slim

COPY ./ ./

RUN docker build --release

CMD ["./target/release/manhattan"]
