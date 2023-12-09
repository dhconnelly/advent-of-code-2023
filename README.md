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

## benchmarks

according to `cargo criterion --output-format=quiet`:

```
day1part1               time:   [48.143 µs 49.193 µs 50.228 µs]
day1part2               time:   [116.90 µs 117.39 µs 117.89 µs]
day2part1               time:   [47.008 µs 47.052 µs 47.095 µs]
day2part2               time:   [68.830 µs 68.913 µs 69.009 µs]
day3part1               time:   [224.04 µs 224.19 µs 224.34 µs]
day3part2               time:   [1.5731 ms 1.5738 ms 1.5743 ms]
day4part1               time:   [103.70 µs 103.94 µs 104.27 µs]
day4part2               time:   [105.12 µs 105.17 µs 105.24 µs]
day5part1               time:   [31.347 µs 31.360 µs 31.374 µs]
day5part2               time:   [35.978 µs 35.998 µs 36.019 µs]
day6part1               time:   [187.01 ns 187.61 ns 188.27 ns]
day6part2               time:   [156.93 ns 157.05 ns 157.18 ns]
day7part1               time:   [306.59 µs 307.66 µs 308.95 µs]
day7part2               time:   [347.45 µs 347.64 µs 347.91 µs]
day8part1               time:   [195.28 µs 196.16 µs 197.12 µs]
day8part2               time:   [396.87 µs 397.91 µs 399.85 µs]
day9part1               time:   [142.90 µs 142.95 µs 143.00 µs]
day9part2               time:   [143.28 µs 143.33 µs 143.38 µs]
```

see `benches/aoc23bench.rs` for the benchmark definitions
