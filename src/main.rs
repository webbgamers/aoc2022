use std::fs;

fn main() {
    let input = fs::read_to_string("./day1.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();
    let input = input.split(|&l| l == "");
    let mut totals = Vec::new();

    for elf in input {
        let mut total = 0;
        for &item in elf {
            total += item.parse::<i32>().unwrap();
        }
        totals.push(total);
    }

    let mut totals2 = totals.clone();

    let mut max = 0;
    for i in totals {
        if i > max { max = i }
    }

    println!("Part 1: {}", max);

    totals2.sort();

    println!("Part 2: {}", totals2[totals2.len()-1] + totals2[totals2.len()-2] + totals2[totals2.len()-3]);
}
