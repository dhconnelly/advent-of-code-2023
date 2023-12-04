# advent-of-code-2023

in rust with `#![no_std]`. no allocations!

## usage

to run the solution for day N on the input `inputs/dayN.txt`:

    cargo run dayN inputs/dayN.txt

to run all solutions:

    ./time_all.sh

to benchmark all solutions:

    cargo bench

to benchmark a specific (day, part):

    cargo bench -- dayNpartM

to generate a flamegraph profile for a specific (day, part):

    cargo flamegraph --bench aoc23bench --root -- --bench dayNpartM --profile-time 30

(the `--root` is required on macOS)
