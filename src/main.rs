use std::time::Instant;
use std::fs;

mod intcode;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
extern crate itertools;
extern crate num;

fn main() 
{
    let now = Instant::now();
    let files = (1..26).map(|i| fs::read_to_string(format!("input/day{}.txt", i)).unwrap_or_default()).collect::<Vec<String>>();
    day1::part1(files[0].trim());
    day1::part2(files[0].trim());
    day2::part1(files[1].trim());
    day2::part2(files[1].trim());
    day3::part1(files[2].trim());
    day3::part2(files[2].trim());
    day4::part1();
    day4::part2();
    day5::part1(files[4].trim());
    day5::part2(files[4].trim());
    day6::part1(files[5].trim());
    day6::part2(files[5].trim());
    day7::part1(files[6].trim());
    day7::part2(files[6].trim());
    day8::part1(files[7].trim());
    day8::part2(files[7].trim());
    day9::part1(files[8].trim());
    day9::part2(files[8].trim());
    let station = day10::part1(files[9].trim());
    day10::part2(files[9].trim(), station);
    day11::part1(files[10].trim());
    day11::part2(files[10].trim());
    day12::part1(files[11].trim());
    day12::part2(files[11].trim());
    day13::part1(files[12].trim());
    day13::part2(files[12].trim());
    day14::part1(files[13].trim());
    day14::part2(files[13].trim());
    println!("Total time: {}ms", now.elapsed().as_millis());
}