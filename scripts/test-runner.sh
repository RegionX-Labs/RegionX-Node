#!/bin/bash

if [ ! -e "regionx-node" ]; then 
    echo "regionx-node binary not found"
    echo "run: cargo build --release --features fast-runtime && cp target/release/regionx-node ."
    exit 1
fi

if [ ! -e "polkadot" ] || [ ! -e "polkadot-parachain" ]; then
    ZOMBIENET_COMMAND="setup polkadot polkadot-parachain"
    if which zombienet-macos &> /dev/null; then
        zombienet-macos $zZOMBIENET_COMMAND
    elif which zombienet-linux &> /dev/null; then
        zombienet-linux $ZOMBIENET_COMMAND
    elif which zombienet &> /dev/null; then
        zombienet $ZOMBIENET_COMMAND
    else
        echo "Zombienet couldn't be located"
    fi
fi

export PATH=$PWD:$PATH

npm run build

zombienet-linux -p native test $1
