#!/bin/bash
# scrappy.sh - Links for Uni PC (Ubuntu)

echo " --- Installing Rust + Tools --- "
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# KEPT Just in Case
#echo " --- Downloading Llama.cpp Server --- "
#curl -L https://github.com/ggml-org/llama.cpp/releases/download/b7836/llama-b7836-bin-ubuntu-x64.zip -o llama.zip

#unzip -o llama.zip -d llama_bin
#cd llama_bin/build/bin
#chmod +x llama-server