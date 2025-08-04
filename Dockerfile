
FROM mcr.microsoft.com/devcontainers/base:ubuntu

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    libudev-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.3/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

RUN cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
