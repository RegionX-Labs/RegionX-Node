#!/bin/bash

# Define the array of module names
modules=(
    "frame_system"
    "cumulus_pallet_parachain_system"
    "pallet_balances"
    "pallet_session"
    "pallet_multisig"
    "pallet_timestamp"
    "pallet_utility"
    "pallet_sudo"
    "pallet_proxy"
    "pallet_preimage"
    "pallet_collator_selection"
    "pallet_message_queue"
    "cumulus_pallet_xcmp_queue"
    "pallet_regions"
    "pallet_market"
    "pallet_processor"
    "pallet_orders"
    "pallet_ismp"
    "ismp_parachain"
    "pallet_mmr_tree"
)

# Iterate through each module and run the benchmark command
for module_name in "${modules[@]}"; do
    ./target/release/regionx-node benchmark pallet \
    --chain regionx-kusama-dev \
    --pallet ${module_name} \
    --steps 20 \
    --repeat 50 \
    --output ./runtime/kusama/src/weights/ \
    --header ./config/HEADER-GPL3 \
    --template ./config/runtime-weight-template.hbs \
    --extrinsic=* \
    --wasm-execution=compiled \
    --heap-pages=4096
done
