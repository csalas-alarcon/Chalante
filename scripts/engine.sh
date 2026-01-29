#!/usr/bin/env bash
# engine.sh - Downloads the Inference Engine

set -e #Exit if Error

# Clone the Repo
echo "--- Cloning the llama.cpp Repo ---"
git clone https://github.com/ggml-org/llama.cpp

# MOVE INTO FOLDER (THIS WON'T HAPPEN AGAIN)
cd llama.cpp

# CMake Config
echo "--- Making the CMake Config ---"
cmake -B build
# Build
echo "--- Compiling llama.cpp ---"
cmake --build build --config Release -j 8