pub fn solve(input: String) -> (Option<isize>, Option<isize>) {
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> Option<isize> {
    // Organize input
    let input = input.lines().collect::<Vec<_>>();
    let input = input.split(|&l| l.is_empty());
    let mut totals = Vec::new();

    // Parse and add numbers
    for elf in input {
        let mut total = 0;
        for &item in elf {
            total += item.parse::<isize>().unwrap();
        }
        totals.push(total);
    }

    // Find highest number
    let mut max = 0;
    for i in totals {
        if i > max {
            max = i
        }
    }
    Some(max)
}

fn part2(input: &str) -> Option<isize> {
    // Organize input
    let input = input.lines().collect::<Vec<_>>();
    let input = input.split(|&l| l.is_empty());
    let mut totals = Vec::new();

    // Parse and add numbers
    for elf in input {
        let mut total = 0;
        for &item in elf {
            total += item.parse::<isize>().unwrap();
        }
        totals.push(total);
    }

    // Sort and add top 3 numbers
    totals.sort_unstable();
    Some(totals[totals.len() - 1] + totals[totals.len() - 2] + totals[totals.len() - 3])
}
