FROM rust:latest
COPY . .
RUN cargo build --release
WORKDIR /home
EXPOSE 8000
CMD ["target/release/moka-web"]