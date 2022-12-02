pub fn solve(input: String) -> (Option<isize>, Option<isize>) {
	(part1(&input), part2(&input))
}

fn part1(input: &String) -> Option<isize> {
    // Organize input
	let input = input.lines().map(|l| {
        let mut l = l.split_whitespace();
        (l.next().expect("bad"), l.next().expect("bad"))
    });
    
    // Calculate score
    let mut score = 0;
    for i in input {
        score += match i.0 {
            "A" => match i.1 {
                "X" => 4,
                "Y" => 8,
                "Z" => 3,
                _ => panic!("bad")
            },
            "B" => match i.1 {
                "X" => 1,
                "Y" => 5,
                "Z" => 9,
                _ => panic!("bad")
            },
            "C" => match i.1 {
                "X" => 7,
                "Y" => 2,
                "Z" => 6,
                _ => panic!("bad")
            },
            _ => panic!("bad")
        }
    }

    Some(score)
}

fn part2(input: &String) -> Option<isize> {
    // Organize input
	let input = input.lines().map(|l| {
        let mut l = l.split_whitespace();
        (l.next().expect("bad"), l.next().expect("bad"))
    });

    // Calculate score
    let mut score = 0;
    for i in input {
        score += match i.0 {
            "A" => match i.1 {
                "X" => 3+0,
                "Y" => 1+3,
                "Z" => 2+6,
                _ => panic!("bad")
            },
            "B" => match i.1 {
                "X" => 1+0,
                "Y" => 2+3,
                "Z" => 3+6,
                _ => panic!("bad")
            },
            "C" => match i.1 {
                "X" => 2+0,
                "Y" => 3+3,
                "Z" => 1+6,
                _ => panic!("bad")
            },
            _ => panic!("bad")
        }
    }

    Some(score)
}
