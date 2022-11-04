FROM mcr.microsoft.com/vscode/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends pandoc python3-dev python3-venv python3-pip

USER vscode

RUN cargo install just
