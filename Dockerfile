# Use the Rust base image
FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef 

RUN apt-get update && apt-get install git

WORKDIR home

COPY . .
RUN rm -rf resources
RUN git clone https://github.com/Moka-Reads/Moka-Resources.git resources


# Build your Rust application
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

# Set the entry point to run your application
CMD ["./target/release/moka-web"]
