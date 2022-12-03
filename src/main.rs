use std::env;
use std::fs;

// Define each day as a module here
mod day01;
mod day02;
mod day03;

fn main() {
    let day = match env::args().nth(1) {
        Some(day) => day,
        None => {
            println!("You must specify a day.");
            return;
        }
    };
    let day = match day.parse::<usize>() {
        Ok(day) => day,
        Err(_) => {
            println!("Enter a vaild day.");
            return;
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

        _ => (String::from("Not implemented yet"), String::from("Not implemented yet")),
    };

    println!("\nDay {} Part 1:", day);
    println!("{}\n", p1);
    println!("Day {} Part 2:", day);
    println!("{}\n", p2);
}

pub fn get_input(day: &usize) -> Option<String> {
    let fp = format!("input/day{:02}.txt", day);
    match fs::read_to_string(&fp) {
        Ok(input) => Some(input),
        Err(error) => {
            println!("Failed to load input file '{}': {}", &fp, error);
            None
        }
    }
}
