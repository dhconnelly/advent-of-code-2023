# advent-of-code-2023

in rust with `#![no_std]`. no allocations!

(except for day21part2 💀💀💀)

the binary crate (`src/main.rs`) uses std to get command-line flags and read
the input. the library crate sets `#![no_std]` in `src/lib.rs` (and pulls in
the rest of the code as modules).

## prerequisites

sdl2 (for the visualization).

for mac os on apple silicon:

    brew install sdl2

    # add to your bashrc or profile:
    export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"

## usage

to run the solution for day N on the input `inputs/dayN.txt`:

    cargo run dayN inputs/dayN.txt

to run day 21:

    cargo run --release day21 inputs/day21.txt

to benchmark all solutions:

    cargo bench

to benchmark a specific (day, part):

    cargo bench -- dayNpartM

to generate a flamegraph profile for a specific (day, part) (make sure
you have installed `flamegraph`):

    cargo flamegraph --bench aoc23bench --root -- --bench dayNpartM --profile-time 30

(the `--root` is required on macOS)

to run the visualization for day 10:

    cargo run --release --bin day10viz

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
day12part1              time:   [8.4260 ms 8.4336 ms 8.4417 ms]
day12part2              time:   [10.473 ms 10.490 ms 10.510 ms]
day13part1              time:   [53.363 µs 53.548 µs 53.743 µs]
day13part2              time:   [56.276 µs 56.422 µs 56.600 µs]
day14part1              time:   [47.485 µs 47.589 µs 47.720 µs]
day14part2              time:   [28.786 ms 28.873 ms 28.983 ms]
day15part1              time:   [120.98 µs 123.51 µs 126.54 µs]
day15part2              time:   [229.38 µs 231.03 µs 233.15 µs]
day16part1              time:   [611.39 µs 611.62 µs 611.85 µs]
day16part2              time:   [186.28 ms 186.37 ms 186.46 ms]
day17part1              time:   [25.439 ms 25.457 ms 25.478 ms]
day17part2              time:   [46.480 ms 46.500 ms 46.522 ms]
day18part1              time:   [69.633 µs 69.689 µs 69.748 µs]
day18part2              time:   [69.728 µs 69.770 µs 69.813 µs]
day19part1              time:   [198.68 µs 199.32 µs 199.96 µs]
day19part2              time:   [187.24 µs 189.82 µs 193.09 µs]
day20part1              time:   [3.9941 ms 3.9958 ms 3.9975 ms]
day20part2              time:   [66.707 ms 66.780 ms 66.885 ms]
day22part1              time:   [18.247 ms 18.271 ms 18.310 ms]
day22part2              time:   [95.162 ms 95.252 ms 95.395 ms]
```

see `benches/aoc23bench.rs` for the benchmark definitions.
