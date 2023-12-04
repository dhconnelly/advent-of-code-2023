# advent-of-code-2023

in rust with `#![no_std]`. no allocations!

the binary crate (`src/main.rs`) uses std to get command-line flags and read
the input. the library crate sets `#![no_std]` in `src/lib.rs` (and pulls in
the rest of the code as modules).

## usage

to run the solution for day N on the input `inputs/dayN.txt`:

    cargo run dayN inputs/dayN.txt

to benchmark all solutions:

    cargo bench

to benchmark a specific (day, part):

    cargo bench -- dayNpartM

to generate a flamegraph profile for a specific (day, part):

    cargo flamegraph --bench aoc23bench --root -- --bench dayNpartM --profile-time 30

(the `--root` is required on macOS)
