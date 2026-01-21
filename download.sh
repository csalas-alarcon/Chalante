#!/bin/bash

echo " --- Installing Rust + Tools --- "
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo " --- Downloading Llama.cpp Server --- "
curl -L https://github.com/ggerganov/llama.cpp/releases/download/b4400/llama-b4400-bin-ubuntu-x64.zip -o llama.zip

unzip -o llama.zip -d llama_bin
cd llama_bin/build/bin
chmod +x llama-server