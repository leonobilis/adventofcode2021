use std::fs;
use std::cmp::{min, max};
use regex::Regex;


#[derive(Clone)]
struct Coord {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32)
}

impl Coord {
    fn count(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) as i64 * (self.y.1 - self.y.0 + 1) as i64 * (self.z.1 - self.z.0 + 1) as i64
    }
}

#[derive(Clone)]
enum Cube {
    On(Coord),
    Off(Coord)
}

impl Cube {
    fn coord(&self) -> &Coord {
        match self {
            Cube::On(coord) => coord,
            Cube::Off(coord) => coord 
        }
    }

    fn intersection(&self, other: &Cube) -> Option<Cube> {
        let x = (max(self.coord().x.0, other.coord().x.0), min(self.coord().x.1, other.coord().x.1));
        let y = (max(self.coord().y.0, other.coord().y.0), min(self.coord().y.1, other.coord().y.1));
        let z = (max(self.coord().z.0, other.coord().z.0), min(self.coord().z.1, other.coord().z.1));

        if x.0 > x.1 || y.0 > y.1 || z.0 > z.1 {
            return None;
        }

        match self {
            Cube::On(_) => Some(Cube::Off(Coord{x, y, z})),
            Cube::Off(_) => Some(Cube::On(Coord{x, y, z}))
        }
    }

    fn count(&self) -> i64 {
        match self {
            Cube::On(coord) => coord.count(),
            Cube::Off(coord) => -coord.count()
        }
    }
}

fn parse_input(inp: &String) -> Vec<Cube> {
    let re = Regex::new(r"^(on|off) x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)$").unwrap();
    inp.lines()
        .map(|line| {
            let c = re.captures(&line).unwrap();
            let coord = Coord {
                x: (c[2].parse().unwrap(), c[3].parse().unwrap()),
                y: (c[4].parse().unwrap(), c[5].parse().unwrap()),
                z: (c[6].parse().unwrap(), c[7].parse().unwrap())
            };
            match &c[1] {
                "on" => Cube::On(coord),
                "off" => Cube::Off(coord),
                _ => panic!("Expected one of ('on', 'off')")
            }
        }).collect()
}

fn p1(cubes: &Vec<Cube>) -> i64 {
    let mut new_cubes = Vec::<Cube>::new();
    cubes.iter()
        .take_while(|c| c.coord().x.0.abs() <= 50)
        .for_each(|c| {
            let very_new_cubes: Vec<Cube> = new_cubes.iter().filter_map(|nc| nc.intersection(&c)).collect();
            new_cubes.extend(very_new_cubes);
            if let Cube::On(_) = c {
                new_cubes.push(c.clone());
            }
        });
    new_cubes.iter().map(|c| c.count()).sum()
}

fn p2(cubes: &Vec<Cube>) -> i64 {
    let mut new_cubes = Vec::<Cube>::new();
    cubes.iter()
        .for_each(|c| {
            let very_new_cubes: Vec<Cube> = new_cubes.iter().filter_map(|nc| nc.intersection(&c)).collect();
            new_cubes.extend(very_new_cubes);
            if let Cube::On(_) = c {
                new_cubes.push(c.clone());
            }
        });
    new_cubes.iter().map(|c| c.count()).sum()
}

fn main() {
    let inp = fs::read_to_string("input.txt").unwrap();
    let cubes = parse_input(&inp);
    println!("Part 1: {}", p1(&cubes));
    println!("Part 2: {}", p2(&cubes));
}
