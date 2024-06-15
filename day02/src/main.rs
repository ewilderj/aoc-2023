use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    id: u32,
    max_r: u32,
    max_g: u32,
    max_b: u32,
}

fn parse_game(l: &str) -> Game {
    if let Some((t_id, rest)) = l[5..].split_once(": ") {
        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        let id = t_id.parse::<u32>().unwrap();
        for p in rest.split("; ") {
            for d in p.split(", ") {
                if let Some((t_n, color)) = d.split_once(" ") {
                    let n = t_n.parse::<u32>().unwrap();
                    match color {
                        "red" => max_r = max_r.max(n),
                        "green" => max_g = max_g.max(n),
                        "blue" => max_b = max_b.max(n),
                        _ => panic!("No idea"),
                    }
                }
            }
        }
        Game {
            id,
            max_r,
            max_g,
            max_b,
        }
    } else {
        panic!("Couldn't parse line");
    }
}

fn main() {
    let games: Vec<Game> = include_str!("../input.txt")
        .lines()
        .map(|l| parse_game(l))
        .collect();

    let part1: u32 = games
        .iter()
        .filter(|g| g.max_r <= 12 && g.max_g <= 13 && g.max_b <= 14)
        .map(|g| g.id)
        .sum();

    println!("part1: {:?}", part1);

    let part2: u32 = games.iter().map(|g| g.max_r * g.max_g * g.max_b).sum();

    println!("part2: {:?}", part2);

    // part1: 2085
    // part2: 79315
}
