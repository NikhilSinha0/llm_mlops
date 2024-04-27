#!/bin/bash

# This script gets the resources defined at the wget links and puts them in the llm_mlops/model directory
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
wget https://huggingface.co/distilbert-base-cased-distilled-squad/resolve/main/rust_model.ot
wget https://huggingface.co/distilbert-base-cased-distilled-squad/resolve/main/config.json
wget https://huggingface.co/bert-large-cased/resolve/main/vocab.txt
mv rust_model.ot $parent_path/rust_model.ot
mv config.json $parent_path/config.json
mv vocab.txt $parent_path/vocab.txt
