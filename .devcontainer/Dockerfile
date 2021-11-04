FROM mcr.microsoft.com/vscode/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends graphviz pandoc python3-dev python3-venv python3-pip

RUN python3 -m pip install --disable-pip-version-check --quiet scriv \
    && rm -rf ~/.local/pip

USER vscode

RUN cargo install just
