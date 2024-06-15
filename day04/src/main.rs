use std::collections::HashMap;

fn count_dupes(v: &Vec<u32>) -> usize {
    let mut counts = HashMap::new();
    for &n in v {
        *counts.entry(n).or_insert(0) += 1;
    }
    counts.values().filter(|&&count| count > 1).count()
}

fn main() {
    let cards: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (_id, rest) = l[4..].split_once(":").unwrap();
            let (l, r) = rest.split_once(" | ").unwrap();
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .chain(r.split_whitespace().map(|n| n.parse::<u32>().unwrap()))
                .collect::<Vec<u32>>()
        })
        .map(|v| count_dupes(&v))
        .collect();

    let part1: usize = cards
        .iter()
        .map(|&n| if n > 0 { 2usize.pow(n as u32 - 1) } else { 0 })
        .sum();

    println!("part1: {:?}", part1);

    let mut v: Vec<usize> = vec![1; cards.len()];
    for (i, &n) in cards.iter().enumerate() {
        // we bump the additional card numbers for n entries
        for x in 0..n {
            v[x + i + 1] += v[i];
        }
    }

    println!("part2: {:}", v.iter().sum::<usize>());

    // part1: 25651
    // part2: 19499881
}
