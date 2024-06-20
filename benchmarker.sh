#!/bin/bash

# Define the array of module names
modules=("frame-system" "pallet-ballances" "pallet-session" "pallet-multisig" "pallet-proxy" "pallet-timestamp" "pallet-utility" "pallet-sudo" "pallet-proxy" "pallet-collator-selection" "cumulus-pallet-xcmp-queue" "pallet-regions" "pallet-market" "pallet-orders" "pallet-whitelist")  # Replace with your actual module names

# Iterate through each module and run the benchmark command
for module_name in "${modules[@]}"; do
    ./target/release/regionx-node benchmark pallet --chain cocos --pallet ${module_name} --steps 20 --repeat 50 --output ./runtime/cocos/src/weights/${module_name}.rs --header ./config/HEADER-GPL3 --template ./config/frame-weight-template.hbs --extrinsic=* --wasm-execution=compiled --heap-pages=4096
done
