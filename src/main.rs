extern crate core;

mod day_1;
mod day_10;
mod day_13;
mod day_2;
mod day_4;
mod day_5;
mod day_6;
mod day_8;

use crate::day_13::day13;
use day_1::day1;
use day_10::day10;
use day_2::day2;
use day_4::day4;
use day_5::day5;
use day_6::day6;
use day_8::day8;

fn main() {
    println!();
    day1::run();
    day2::run();
    day4::run();
    day5::run();
    day6::run();
    day8::run();
    day10::run();
    day13::run();
}
