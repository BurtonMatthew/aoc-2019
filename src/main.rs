//#![feature(unboxed_closures)]
//#![feature(fn_traits)]

use std::time::Instant;

mod intcode;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
extern crate itertools;

fn main() 
{
    let now = Instant::now();
    day1::part1();
    day1::part2();
    day2::part1();
    day2::part2();
    day3::part1();
    day3::part2();
    day4::part1();
    day4::part2();
    day5::part1();
    day5::part2();
    day6::part1();
    day6::part2();
    day7::part1();
    day7::part2();
    println!("Total time: {}ms", now.elapsed().as_millis());
}