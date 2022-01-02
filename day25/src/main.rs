use std::collections::HashSet;
use std::fs;

type Grid = (HashSet<(u8, u8)>, HashSet<(u8, u8)>, u8, u8);

fn parse_input(inp: &str) -> Grid {
    let (east, south): (Vec<_>, Vec<_>) = inp
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| ((x as u8, y as u8), c))
        })
        .partition(|(_, c)| *c == '>');

    (
        east.iter().map(|(pos, _)| *pos).collect(),
        south.iter().map(|(pos, _)| *pos).collect(),
        inp.lines().count() as u8,
        inp.lines().next().unwrap().len() as u8,
    )
}

fn p1(mut east: HashSet<(u8, u8)>, mut south: HashSet<(u8, u8)>, leny: &u8, lenx: &u8) -> u32 {
    let mut emoves = 1;
    let mut smoves = 1;
    let mut steps = 0;

    while emoves != 0 || smoves != 0 {
        let (new_east, old_east): (Vec<_>, Vec<_>) = east.iter().partition(|(x, y)| {
            !east.contains(&((x + 1) % lenx, *y)) && !south.contains(&((x + 1) % lenx, *y))
        });
        emoves = new_east.len();
        east = new_east
            .iter()
            .map(|(x, y)| ((x + 1) % lenx, *y))
            .chain(old_east.into_iter())
            .collect();

        let (new_south, old_south): (Vec<_>, Vec<_>) = south.iter().partition(|(x, y)| {
            !east.contains(&(*x, (y + 1) % leny)) && !south.contains(&(*x, (y + 1) % leny))
        });
        smoves = new_south.len();
        south = new_south
            .iter()
            .map(|(x, y)| (*x, (y + 1) % leny))
            .chain(old_south.into_iter())
            .collect();

        steps += 1;
    }

    steps
}

fn main() {
    let inp = fs::read_to_string("input.txt").unwrap();
    let (east, south, leny, lenx) = parse_input(&inp);
    println!("Part 1: {}", p1(east, south, &leny, &lenx));
}
