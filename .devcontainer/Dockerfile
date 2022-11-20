FROM mcr.microsoft.com/devcontainers/rust:latest

RUN set -ex \
    && wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- \
    && mv trunk /usr/local/cargo/bin/

WORKDIR /app