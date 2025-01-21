# Builder stage
FROM rust:1.81.0 AS builder

RUN rustup target add x86_64-unknown-linux-gnu

# Install required tools
RUN rustup component add llvm-tools rustc-dev && \
    apt-get update && apt-get install -y ca-certificates curl && \
    update-ca-certificates

WORKDIR /usr/src/app

# Download proving key
RUN curl -L "https://storage.googleapis.com/zk-vm/sp1/email_with_regex.bin" -o /usr/local/bin/proving_key.bin

# Copy entire workspace for dependencies
COPY . .

# Build prover
WORKDIR /usr/src/app/prover
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Use Debian Bookworm base image for the final container
FROM debian:bookworm-slim

# Install OpenSSL 3 and other runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

ENV PROVING_KEY_PATH=/usr/local/bin/proving_key.bin
ENV SP1_PROVER=network

WORKDIR /app

# Copy the binary and proving key from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/prover .
COPY --from=builder /usr/local/bin/proving_key.bin ${PROVING_KEY_PATH}

# Expose the application port
EXPOSE 8081

# Start the application
CMD ["./prover"]
