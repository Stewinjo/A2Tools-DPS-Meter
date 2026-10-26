FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

# Ubuntu 22.04 matches the glibc baseline of the Linux packages, so a binary
# built in this container runs on 22.04 and newer. Node 24 matches .nvmrc.
# Rust comes from rust-toolchain.toml at the repo root.

ENV DEBIAN_FRONTEND=noninteractive \
    PATH="/home/vscode/.cargo/bin:/home/vscode/.local/bin:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
        libwebkit2gtk-4.1-dev \
        build-essential \
        curl \
        wget \
        file \
        libxdo-dev \
        libssl-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        libpcap-dev \
        pkg-config \
        python3-pip \
        python3-venv \
        ca-certificates \
        gnupg \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/* \
    && node --version

USER vscode

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none \
    && pip3 install --user pre-commit

COPY --chown=vscode:vscode rust-toolchain.toml /tmp/toolchain/rust-toolchain.toml
WORKDIR /tmp/toolchain
RUN rustup show && rustup component add rust-src

WORKDIR /home/vscode
