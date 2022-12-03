FROM rust:1.65.0
LABEL maintainer1="m@shomov.spb.ru"
LABEL maintainer2="garipova.gz@mail.ru"

EXPOSE 8080


FROM rust:1.65-slim
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/verilog_synthesis"]
