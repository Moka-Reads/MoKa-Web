# Use the Rust base image
FROM rust:latest

RUN apt-get update && apt-get install git
# Set the working directory to /home/resources
WORKDIR /home

# Create a directory to store SSH keys
RUN mkdir -p /root/.ssh

RUN git clone https://github.com/Moka-Reads/MoKa-Web.git .
RUN rmdir resources
RUN git clone https://github.com/Moka-Reads/Moka-Resources.git resources
RUN cd resources && git clone https://github.com/Moka-Reads/Moka-Articles.git
RUN cd resources && git clone https://github.com/Moka-Reads/Moka-Cheatsheets.git
RUN cd resources && git clone https://github.com/Moka-Reads/Moka-Guides.git
# Build your Rust application
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

# Set the entry point to run your application
CMD ["./target/release/moka-web"]
