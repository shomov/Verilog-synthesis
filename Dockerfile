FROM rust:1.65.0
LABEL maintainer1="m@shomov.spb.ru"
LABEL maintainer2="garipova.gz@mail.ru"

EXPOSE 8080
COPY ./ ./
RUN cargo build --release

FROM rust:1.65-slim
CMD ["./target/release/verilog_synthesis"]
