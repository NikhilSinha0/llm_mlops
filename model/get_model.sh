parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
wget https://huggingface.co/rustformers/pythia-ggml/resolve/main/pythia-1.4b-q4_0-ggjt.bin
mv pythia-1.4b-q4_0-ggjt.bin $parent_path/pythia.bin