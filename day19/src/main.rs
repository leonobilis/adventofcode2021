use std::fs;
use std::ops::{Add, Sub};
use std::cmp::{PartialEq, PartialOrd, Eq, Ord};
use itertools::Itertools;


#[derive(PartialEq, Eq, PartialOrd, Ord, std::hash::Hash, Clone, Copy)]
struct Point {
    x: i32, y: i32, z: i32
}

impl Point {
    fn new() -> Self {
        Point{x: 0, y: 0, z: 0}
    }

    fn from_iter<'a, I: Iterator::<Item=&'a str>>(mut iter: I) -> Self {
        Point{
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
            z: iter.next().unwrap().parse().unwrap()
        }
    }

    fn manhatan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

struct Scanner {
    pos: Point,
    points: Vec<Point>
}

impl Scanner {
    fn rotations(& self) -> Vec<Scanner> {
        vec![
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.x, y:  p.y, z:  p.z}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.x, y: -p.z, z:  p.y}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.x, y: -p.y, z: -p.z}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.x, y:  p.z, z: -p.y}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.x, y: -p.y, z:  p.z}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.x, y: -p.z, z: -p.y}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.x, y:  p.y, z: -p.z}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.x, y:  p.z, z:  p.y}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.z, y:  p.x, z: -p.y}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.y, y:  p.x, z: -p.z}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.z, y:  p.x, z:  p.y}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.y, y:  p.x, z:  p.z}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.z, y: -p.x, z: -p.y}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.y, y: -p.x, z:  p.z}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.z, y: -p.x, z:  p.y}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.y, y: -p.x, z: -p.z}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.y, y: -p.z, z:  p.x}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.z, y: -p.y, z:  p.x}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.y, y:  p.z, z:  p.x}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.z, y:  p.y, z:  p.x}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.z, y:  p.y, z: -p.x}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.y, y:  p.z, z: -p.x}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x: -p.z, y: -p.y, z: -p.x}).collect()},
            Scanner{pos: self.pos, points: self.points.iter().map(|p| Point{x:  p.y, y: -p.z, z: -p.x}).collect()}
        ]
    }

    fn normalize(&self, reference: &Scanner) -> Option<Scanner> {
        let rotations = self.rotations();
        let matched  = rotations.iter()
            .map(|s| (s, s.points.iter()
                .flat_map(|p| reference.points.iter()
                        .map(move |rp| *rp - *p)
                    )
                    .sorted()
                    .group_by(|x| *x)
                    .into_iter()
                    .map(|(k, v)| (k, v.count()))
                    .max_by_key(|(_, c)| *c)
                    .unwrap()
                )
            ).skip_while(|(_, (_, c))| *c < 12).next();
        
        match matched {
            Some((scanner, (diff, _))) => Some(Scanner{
                pos: self.pos + diff,
                points: scanner.points.iter().map(|p| *p + diff).collect()
            }),
            None => None
        }
    }
}

fn parse_input(inp: &String) -> Vec<Scanner> {
    inp.split("\n\n")
        .map(|scanner| Scanner {
            pos: Point::new(),
            points: scanner.split("\n").skip(1).map(|point| Point::from_iter(point.split(","))).collect()
        })
    .collect()
}

fn get_normalized_scanners(mut scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut ref_scanners = Vec::with_capacity(scanners.len()); 
    let mut result_scanners = Vec::with_capacity(scanners.len());

    ref_scanners.push(scanners.pop().unwrap());

    while !scanners.is_empty() {
        let ref_scanner = ref_scanners.pop().unwrap();
        let new_references: Vec<_> = scanners.iter()
            .map(|sc| sc.normalize(&ref_scanner))
            .collect();

        let mut keep = new_references.iter().map(|sc| sc.is_none());
        scanners.retain(|_| keep.next().unwrap());
        
        ref_scanners.extend(new_references.into_iter().filter_map(|nr| nr));
        result_scanners.push(ref_scanner);
    }
    result_scanners.extend(ref_scanners);
    result_scanners
}

fn p1(scanners: &Vec<Scanner>) -> usize {
    scanners.iter().flat_map(|sc| sc.points.iter()).unique().count()
}

fn p2(scanners: &Vec<Scanner>) -> i32 {
    scanners.iter().flat_map(|s1| scanners.iter().map(move |s2| s1.pos.manhatan(&s2.pos))).max().unwrap()
}

fn main() {
    let inp = fs::read_to_string("input.txt").unwrap();
    let scanners = get_normalized_scanners(parse_input(&inp));
    println!("Part 1: {}", p1(&scanners));
    println!("Part 2: {}", p2(&scanners));
}
