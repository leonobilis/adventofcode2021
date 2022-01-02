use std::collections::HashSet;
use std::fs;

struct Image {
    coords: HashSet<(i32, i32)>,
    start: i32,
    end: i32,
}

impl Image {
    fn from_str(s: &str) -> Self {
        let len = s.lines().next().unwrap().len() as i32;
        Image {
            coords: s
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some((x as i32, y as i32))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
            start: 0,
            end: len - 1,
        }
    }

    fn enhance(&self, e_alg: &[bool]) -> Image {
        let start = self.start - 2;
        let end = self.end + 2;
        let mut coords: HashSet<(i32, i32)> = (start..=end)
            .flat_map(|y| {
                (start..=end).filter_map(move |x| {
                    let index: usize = (-1..=1)
                        .rev()
                        .flat_map(|yy| {
                            (-1..=1)
                                .rev()
                                .map(move |xx| self.coords.contains(&(x + xx, y + yy)))
                        })
                        .enumerate()
                        .filter(|(_, coord)| *coord)
                        .map(|(i, _)| 1 << i)
                        .sum();
                    if e_alg[index] {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect();
        if coords.contains(&(start, start)) {
            // Add frame
            coords.extend((start - 3..=end + 3).flat_map(|a| {
                [
                    (a, start - 3),
                    (a, start - 2),
                    (a, start - 1),
                    (a, end + 3),
                    (a, end + 2),
                    (a, end + 1),
                    (start - 3, a),
                    (start - 2, a),
                    (start - 1, a),
                    (end + 3, a),
                    (end + 2, a),
                    (end + 1, a),
                ]
            }));
        }
        Image { coords, start, end }
    }
}

fn parse_input(inp: &str) -> (Vec<bool>, Image) {
    let mut splitted = inp.split("\n\n");
    let e_alg = splitted.next().unwrap().chars().map(|c| c == '#').collect();
    let inp_image = Image::from_str(splitted.next().unwrap());
    (e_alg, inp_image)
}

fn p1(e_alg: &[bool], inp_image: &Image) -> usize {
    inp_image.enhance(e_alg).enhance(e_alg).coords.len()
}

fn p2(e_alg: &[bool], inp_image: &Image) -> usize {
    let mut image = inp_image.enhance(e_alg);
    for _ in 1..50 {
        image = image.enhance(e_alg);
    }
    image.coords.len()
}

fn main() {
    let inp = fs::read_to_string("input.txt").unwrap();
    let (e_alg, inp_image) = parse_input(&inp);
    println!("Part 1: {}", p1(&e_alg, &inp_image));
    println!("Part 2: {}", p2(&e_alg, &inp_image));
}
