FROM mcr.microsoft.com/devcontainers/rust:latest

WORKDIR /workspaces

USER vscode 

ENV NVM_DIR="/home/vscode/.nvm"

RUN sudo sed -i 's|deb.debian.org|mirrors.aliyun.com|g' /etc/apt/sources.list.d/debian.sources && \
    sudo apt update && \
    sudo apt install -y \
    musl-tools \ 
    wget \
    curl \
    vim \
    git && \
    cargo install \
    eza \
    ripgrep\
    git-commitizen && \
    rustup target add aarch64-unknown-linux-musl

ENV NODE_PATH=$NVM_DIR/v22/lib/node_modules
ENV PATH=$NVM_DIR/v22/bin:$PATH

COPY .devcontainer/entrypoint.sh /
