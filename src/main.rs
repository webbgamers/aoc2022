use chrono::prelude::*;
use chrono_tz::EST;
use std::env;
use std::fs;

// Define each day as a module here
mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let day = match env::args().nth(1) {
        Some(day) => match day.parse::<u32>() {
            Ok(day) => day,
            Err(_) => {
                println!("Enter a vaild day.");
                return;
            }
        },
        None => {
            let time = Utc::now().with_timezone(&EST);
            if time.month() == 12 && time.day() <= 25 {
                time.day()
            } else {
                println!("You must specify a day.");
                return;
            }
        }
    };

    if day > 25 {
        println!("Enter a day between 1 and 25.");
        return;
    }

    let input = match get_input(&day) {
        Some(input) => input,
        None => {
            println!("Verify input file and try again.");
            return;
        }
    };

    // Add each day to match statement here
    let (p1, p2) = match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),

        _ => (
            String::from("Not implemented yet"),
            String::from("Not implemented yet"),
        ),
    };

    println!("\nDay {} Part 1:", day);
    println!("{}\n", p1);
    println!("Day {} Part 2:", day);
    println!("{}\n", p2);
}

pub fn get_input(day: &u32) -> Option<String> {
    let fp = format!("input/day{:02}.txt", day);
    match fs::read_to_string(&fp) {
        Ok(input) => Some(input),
        Err(error) => {
            println!("Failed to load input file '{}': {}", &fp, error);
            None
        }
    }
}
