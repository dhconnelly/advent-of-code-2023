use advent_of_code_2023::day1;

fn main() {
    let path = std::env::args().skip(1).next().unwrap();
    let data = std::fs::read_to_string(&path).unwrap();
    println!("{}", day1::part1(&data));
    println!("{}", day1::part2(&data));
}
