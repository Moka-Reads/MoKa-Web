# Use the Rust base image
FROM rust:latest

RUN apt-get update && apt-get install git

RUN git clone --recursive https://github.com/Moka-Reads/MoKa-Web.git .

# Build your Rust application
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

# Set the entry point to run your application
CMD ["./target/release/moka-web"]
