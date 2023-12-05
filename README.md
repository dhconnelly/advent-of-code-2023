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
day3part1               time:   [3.7704 ms 3.7718 ms 3.7734 ms]
day3part2               time:   [1.5731 ms 1.5738 ms 1.5743 ms]
day4part1               time:   [103.70 µs 103.94 µs 104.27 µs]
day4part2               time:   [105.12 µs 105.17 µs 105.24 µs]
day5part1               time:   [31.347 µs 31.360 µs 31.374 µs]
day5part2               time:   [35.978 µs 35.998 µs 36.019 µs]
```

