#!/bin/bash

set -e

cargo build --release

for input in inputs/*.txt; do
    day=$(basename "${input%.txt}")
    time ./target/release/"$day" $input
done
