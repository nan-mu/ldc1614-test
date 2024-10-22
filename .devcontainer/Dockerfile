FROM mcr.microsoft.com/devcontainers/rust:latest

WORKDIR /workspaces

USER vscode 

ENV NVM_DIR="/home/vscode/.nvm"

RUN sudo sed -i 's|deb.debian.org|mirrors.aliyun.com|g' /etc/apt/sources.list.d/debian.sources && \
    sudo apt update && \
    sudo apt install -y \
    build-essential \
    musl-tools \
    # bison \
    # flex \
    # texinfo \
    wget \
    git && \
    # curl -L https://github.com/lovell/aarch64-linux-musl-crosstools/archive/refs/heads/main.zip -o /opt/main.zip && \
    # unzip /opt/main.zip -d /opt && \
    if [ "$proxy"x = "none"x ]; then \
    rustup target add aarch64-unknown-linux-musl && \
    cargo install eza ripgrep && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash ; \
    else \
    http_proxy=$proxy \
    https_proxy=$proxy \
    rustup target add aarch64-unknown-linux-musl && \
    http_proxy=$proxy \
    https_proxy=$proxy \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash ;\
    fi && \
    if [ "$proxy"x = "none"x ]; then \
    bash -c 'source $NVM_DIR/nvm.sh && \
    nvm alias default 22 && \
    nvm install 22 && \
    nvm use default && \
    npm config set registry https://registry.npmmirror.com/ && \
    npm install -g commitizen && \
    commitizen init cz-conventional-changelog --save-dev --save-exact' ; \
    else \
    http_proxy=$proxy \
    https_proxy=$proxy \
    bash -c 'source $NVM_DIR/nvm.sh && \
    nvm alias default 22 && \
    nvm install 22 && \
    nvm use default && \
    npm config set registry https://registry.npmmirror.com/ && \
    npm install -g commitizen && \
    commitizen init cz-conventional-changelog --save-dev --save-exact' ; \
    fi

ENV NODE_PATH=$NVM_DIR/v22/lib/node_modules
ENV PATH=$NVM_DIR/v22/bin:$PATH

COPY .devcontainer/entrypoint.sh /