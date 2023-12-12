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

to generate a flamegraph profile for a specific (day, part) (make sure
you have installed `flamegraph`):

    cargo flamegraph --bench aoc23bench --root -- --bench dayNpartM --profile-time 30

(the `--root` is required on macOS)

## benchmarks

according to `cargo bench` on my 2022 m2 macbook air, charging:

```
day1part1               time:   [27.278 µs 27.823 µs 28.286 µs]
day1part2               time:   [66.983 µs 67.018 µs 67.062 µs]
day2part1               time:   [27.174 µs 27.187 µs 27.205 µs]
day2part2               time:   [40.147 µs 40.167 µs 40.189 µs]
day3part1               time:   [127.72 µs 127.77 µs 127.83 µs]
day3part2               time:   [908.11 µs 908.37 µs 908.71 µs]
day4part1               time:   [59.585 µs 59.631 µs 59.700 µs]
day4part2               time:   [61.097 µs 61.386 µs 61.763 µs]
day5part1               time:   [18.027 µs 18.040 µs 18.057 µs]
day5part2               time:   [20.802 µs 20.809 µs 20.817 µs]
day6part1               time:   [170.62 ns 170.84 ns 171.08 ns]
day6part2               time:   [156.20 ns 156.34 ns 156.54 ns]
day7part1               time:   [303.21 µs 303.45 µs 303.70 µs]
day7part2               time:   [353.91 µs 355.33 µs 356.85 µs]
day8part1               time:   [199.51 µs 199.58 µs 199.66 µs]
day8part2               time:   [382.16 µs 382.37 µs 382.62 µs]
day9part1               time:   [82.521 µs 82.613 µs 82.754 µs]
day9part2               time:   [82.364 µs 82.427 µs 82.521 µs]
day10part1              time:   [2.8895 ms 2.8903 ms 2.8912 ms]
day10part2              time:   [5.3630 ms 5.3644 ms 5.3658 ms]
day11part1              time:   [2.5524 ms 2.5604 ms 2.5693 ms]
day11part2              time:   [2.5535 ms 2.5609 ms 2.5690 ms]
day12part1              time:   [8.7509 ms 8.7902 ms 8.8330 ms]
day12part2              time:   [12.602 ms 12.641 ms 12.684 ms]
```

see `benches/aoc23bench.rs` for the benchmark definitions.
