use core::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
struct Mapping {
    dest: u64,
    src: u64,
    length: u64,
}

// when we sort, order by source range to make this easy
impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.src.cmp(&other.src))
    }
}

impl Mapping {
    // checks to see if a number is in range, and if so
    // returns the translated number
    fn translate(&self, n: u64) -> Option<u64> {
        if n >= self.src && n < (self.src + self.length) {
            return Some(n - self.src + self.dest);
        }
        None
    }
}

fn process(seeds: &Vec<u64>, mappings: &Vec<Vec<Mapping>>) -> u64 {
    let mut ss = seeds.clone();

    for mps in mappings {
        for (i, &seed) in ss.clone().iter().enumerate() {
            for m in mps.iter() {
                if let Some(ns) = m.translate(seed) {
                    ss[i] = ns;
                    break;
                }
            }
        }
    }
    ss.sort();
    ss[0]
}

fn main() {
    let mut s = include_str!("../input.txt").split("\n\n");
    let seeds: Vec<u64> = s
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut mappings: Vec<Vec<Mapping>> = Vec::new();
    // now let's cycle through each map
    for (_i, l) in s.enumerate() {
        let (_, map_data) = l.split_once("\n").unwrap();
        let mut mps = map_data
            .lines()
            .map(|d| {
                let (dest, src, length) = d
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                Mapping { dest, src, length }
            })
            .collect::<Vec<Mapping>>();
        mps.sort();
        mappings.push(mps);
    }

    println!("part1: {:}", process(&seeds, &mappings));

    // BRUTE FORCE APPROACH
    //
    let mut seeds2: Vec<u64> = Vec::new();
    for i in 0..(seeds.len() / 2) {
        seeds2.extend(
            (seeds[i * 2]..(seeds[i * 2] + seeds[i * 2 + 1]))
                .into_iter()
                .collect::<Vec<u64>>(),
        );
    }

    println!("computing {:} seeds", seeds2.len());
    println!("part2: {:}", process(&seeds2, &mappings));

    // check out https://www.reddit.com/r/adventofcode/comments/18b4b0r/2023_day_5_solutions/
    // for smarter ways to do this. not gonna bother here cos brute force
    // only took 1.5 minutes on my machine.
}

// part1: 1181555926
// part2: 37806486
