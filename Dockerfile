# Use the Rust base image
FROM rust:latest

RUN apt-get update && apt-get install git
# Set the working directory to /home/resources
WORKDIR /home

RUN git clone https://github.com/Moka-Reads/MoKa-Web.git .

# Build your Rust application
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

# Set the entry point to run your application
CMD ["./target/release/moka-web"]
