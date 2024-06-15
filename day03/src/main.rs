use regex::Regex;
use std::collections::HashMap;

type GearMap = HashMap<(usize, usize), Vec<u32>>;

fn main() {
    let board: Vec<&str> = include_str!("../input.txt").lines().collect();
    let max_x = board[0].len() - 1;
    let max_y = board.len() - 1;
    let re = Regex::new(r"[0-9]+").unwrap();

    // searches within a rectangle for symbols that indicate a partno
    // also logs if there's a gear * the partno that's adjacent
    fn has_sym(
        board: &Vec<&str>,
        gears: &mut GearMap,
        pno: u32, // part no
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
    ) -> bool {
        let mut r = false;
        for y in y0..y1 + 1 {
            let s = &(board[y])[x0..x1 + 1];
            for (i, c) in s.char_indices() {
                if !c.is_numeric() && c != '.' {
                    r = true;
                    if c == '*' {
                        let g_x = x0 + i;
                        gears.entry((g_x, y)).or_insert_with(Vec::new).push(pno);
                    }
                }
            }
        }
        r
    }

    let mut part1 = 0;
    let mut gears = GearMap::new();

    for y in 0..max_y + 1 {
        for n in re.captures_iter(board[y]) {
            let m = n.get(0).unwrap();
            let pno = m.as_str().parse::<u32>().unwrap();

            // make a bounding box of search area
            let x0 = if m.start() == 0 { 0 } else { m.start() - 1 };
            let y0 = if y == 0 { 0 } else { y - 1 };
            let x1 = max_x.min(m.end());
            let y1 = max_y.min(y + 1);

            // if we found a symbol, it's a part no
            // has_sym also logs positions of gears as it works
            if has_sym(&board, &mut gears, pno, x0, y0, x1, y1) {
                part1 = part1 + pno;
            }
        }
    }
    println!("part1: {:?}", part1);

    // just look for gears with >1 partno and multiply
    let part2: u32 = gears
        .iter()
        .map(|(_, v)| if v.len() > 1 { v.iter().product() } else { 0 })
        .sum();

    println!("part2: {:?}", part2);

    // part1: 554003
    // part2: 87263515
}
