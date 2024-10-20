FROM mcr.microsoft.com/devcontainers/rust:latest

WORKDIR /workspaces

USER vscode 

RUN sudo apt update && \
    sudo apt install -y \
        git \
        gcc-aarch64-linux-gnu && \
    rustup target add aarch64-unknown-linux-musl

RUN git config --global http.proxy "" && \
    git config --global https.proxy "" && \
    git config --global --add safe.directory /workspaces && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash

RUN export NVM_DIR="$HOME/.nvm" && \
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" && \
    [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion" && \
    nvm install 22 && \
    npm install -g commitizen && \
    commitizen init cz-conventional-changelog --save-dev --save-exact

RUN git config --global --add safe.directory /workspaces