#!/bin/bash

set -ex

cargo build --release

function execute() {
    for input in inputs/*.txt; do
        day=$(basename "${input%.txt}")
        ./target/release/advent-of-code-2023 "$day" $input
    done
}

time execute
