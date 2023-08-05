# Use the Rust base image
FROM rust:latest

# Install git
RUN apt-get update && apt-get install -y git

# Set the working directory to /home/resources
WORKDIR /home

# Clone the repository into /home/resources
RUN git clone https://github.com/Moka-Reads/Moka-Resources.git resources

COPY . .

# Build your Rust application
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

# Set the entry point to run your application
CMD ["./target/release/moka-web"]
