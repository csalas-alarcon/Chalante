#!/bin/bash

echo " --- Downloading Tiny Models (No Token Required) --- "
mkdir -p models

# 1. Phi2 - Super Big
curl -L "https://huggingface.co/TheBloke/phi-2-GGUF/resolve/main/phi-2.Q4_K_M.gguf?download=true" -o models/phi2.gguf
# 2. Qwen2.5 (0.5B parameters) - Surprisingly smart for its size
curl -L "https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q8_0.gguf?download=true" -o models/qwen.gguf

# 3. H2O-Dabube - Added it just to get three, he's adopted.
curl -L "https://huggingface.co/h2oai/h2o-danube3-500m-chat-GGUF/resolve/main/h2o-danube3-500m-chat-Q8_0.gguf?download=true" -o models/danube.gguf
echo " --- Starting LLAMA Server --- "
PROJECT_ROOT=$(pwd)
./llama_bin/build/bin/llama-server \
    -m "$PROJECT_ROOT/parlante/models/qwen.gguf" \
    --port 11343 > llama_server.log 2>&1 &

SERVER_PID=$!

echo " --- Launching Chalante TUI --- "
cd parlante
cargo run

#echo " --- Shutting down AI Server --- "
#kill $SERVER_PID