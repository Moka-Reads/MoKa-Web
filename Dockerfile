# Use the Rust base image
FROM rust:latest

# Set the working directory to /home/resources
WORKDIR /home

COPY . .

# Build your Rust application
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

# Set the entry point to run your application
CMD ["./target/release/moka-web"]
