use std::collections::HashSet;

pub fn solve(input: String) -> (Option<isize>, Option<isize>) {
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> Option<isize> {
    let input = input.lines().map(|l| {
        let mut first = HashSet::new();
        let mut second = HashSet::new();
        for (i, c) in l.chars().enumerate() {
            if i < l.len() / 2 {
                first.insert(c);
            } else {
                second.insert(c);
            }
        }
        (first, second)
    });

    let mut results = Vec::new();
    for i in input {
        let mut common = i.0.intersection(&i.1);
        results.push(common.next().cloned().unwrap());
    }

    let mut total = 0;
    for r in results {
        total += "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .position(|c| c == r)
            .unwrap()
            + 1;
    }

    Some(total as isize)
}

fn part2(input: &str) -> Option<isize> {
    let mut input = input.lines().map(|l| l.chars().collect::<HashSet<_>>());

    let mut results = Vec::new();
    while 0 < input.clone().count() {
        // All my homies hate the borrow checker
        let e1 = input.next().unwrap();
        let e2 = input.next().unwrap();
        let e3 = input.next().unwrap();
        let common = e1.intersection(&e2).copied().collect::<HashSet<_>>();
        let mut common = common.intersection(&e3);

        results.push(common.next().cloned().unwrap());
    }

    let mut total = 0;
    for r in results {
        total += "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .position(|c| c == r)
            .unwrap()
            + 1;
    }

    Some(total as isize)
}
