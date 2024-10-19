FROM mcr.microsoft.com/devcontainers/rust:latest

WORKDIR /workspaces

RUN sudo apt install git

RUN git config --global http.proxy "" && \
    git config --global https.proxy "" && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash

RUN
    NVM_DIR="$HOME/.nvm" nvm install 22 && \
    npm install -g commitizen && \
    commitizen init cz-conventional-changelog --save-dev --save-exact