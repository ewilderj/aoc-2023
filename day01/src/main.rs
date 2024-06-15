use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MY_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
}

fn replace_numbers(s: &str) -> String {
    let mut o = String::from(s);

    fn chop(s: &String, i: usize, k: &str, v: &str) -> String {
        let mut r = String::new();
        let n = i + k.len();
        r.push_str(&s[0..i]);
        r.push_str(&v);
        r.push_str(&s[n..]);
        r
    }

    // look for the left most digit, either numeric or written
    let mut i = 0;
    'outer: while i < o.len() {
        if o[i..].starts_with(char::is_numeric) {
            break;
        }
        for (k, v) in MY_MAP.iter() {
            // if it's written, replace with the digit
            if o[i..].starts_with(k) {
                o = chop(&o, i, &k, &v);
                break 'outer;
            }
        }
        i = i + 1;
    }

    // look for the right most digit, either numeric or written
    i = o.len() - 1;
    'outer2: loop {
        if o[i..].starts_with(char::is_numeric) {
            break;
        }
        for (k, v) in MY_MAP.iter() {
            if o[i..].starts_with(k) {
                o = chop(&o, i, &k, &v);
                break 'outer2;
            }
        }
        if i == 0 {
            break;
        } else {
            i = i - 1;
        }
    }

    o
}

fn main() {
    let part1: u32 = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
        .map(|l| String::from_iter([l.first().unwrap(), l.last().unwrap()]))
        .map(|s| s.parse::<u32>().unwrap())
        .sum();

    println!("part1: {:?}", part1);

    let part2: u32 = include_str!("../input.txt")
        .lines()
        .map(|l| {
            replace_numbers(l)
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<_>>()
        })
        .map(|l| String::from_iter([l.first().unwrap(), l.last().unwrap()]))
        .map(|s| s.parse::<u32>().unwrap())
        .sum();

    println!("part2: {:?}", part2);
}
