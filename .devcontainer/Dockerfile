# syntax=docker/dockerfile:1

FROM mcr.microsoft.com/devcontainers/rust:latest

# NOTE:
# `cargo install` のインストール先がユーザのホームディレクトリ配下となるよう、
# vscode ユーザへあらかじめ切り替えておく
USER vscode

# Setup for Yew
RUN <<EOF
# Install trunk
cargo install trunk

# Add target for wasm
rustup target add wasm32-unknown-unknown
EOF
