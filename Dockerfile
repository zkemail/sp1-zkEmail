# Builder stage
FROM rust:1.81.0 as builder
RUN rustup component add llvm-tools rustc-dev
RUN apt-get update && apt-get install -y curl

WORKDIR /usr/src/app
# Download proving key
RUN curl -L "https://storage.googleapis.com/zk-vm/sp1/email_with_regex.bin" -o /usr/local/bin/proving_key.bin

# Copy entire workspace for dependencies
COPY . .

# Build prover
RUN cd prover && cargo build --release

# Build args for build-time
ARG SP1_PROVER
ARG NETWORK_PRIVATE_KEY
ARG NETWORK_RPC_URL

# Runtime environment variables
ENV SP1_PROVER=${SP1_PROVER}
ENV NETWORK_PRIVATE_KEY=${NETWORK_PRIVATE_KEY}
ENV NETWORK_RPC_URL=${NETWORK_RPC_URL}

CMD ["cd", "prover", "&&", "cargo", "run", "--release"]