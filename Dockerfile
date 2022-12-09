FROM rust:latest as build
 
RUN mkdir /rust-build
COPY ./ /rust-build
WORKDIR /rust-build
RUN cargo build --release
 
FROM alt:latest
EXPOSE 8080:8080
RUN mkdir /app
COPY --from=build /rust-build/target/release/verilog_synthesis /app
CMD ["/app/verilog_synthesis"]
