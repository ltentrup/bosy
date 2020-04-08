# Set the base image to Rust
FROM rust:latest

# Update the repository sources list
RUN apt-get update -yqq \
    && apt-get install -yqq --no-install-recommends automake autoconf build-essential cmake llvm-6.0-dev libclang-6.0-dev clang-6.0 python

# Install benchmarking tool
RUN cargo install hyperfine

# Add dummy git config
RUN git config --global user.email "you@example.com" \
    && git config --global user.name "Your Name"

COPY . /bosy

RUN cd /bosy && make

RUN cd /bosy && cargo test && cargo build --release