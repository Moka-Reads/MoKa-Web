FROM rust:latest
WORKDIR /home
COPY . .
RUN cargo build --release
EXPOSE 8080
CMD ["target/release/moka-web"]