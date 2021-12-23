use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: i32, y: i32, z: i32
}

impl Point {
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

#[derive(Debug)]
struct Scanner {
    points: Vec<Point>
}

impl Scanner {
    fn x_vector(&self) -> Vec<i32> {
        self.points.iter().map(|p| p.x).collect()
    }
    fn y_vector(&self) -> Vec<i32> {
        self.points.iter().map(|p| p.y).collect()
    }
    fn z_vector(&self) -> Vec<i32> {
        self.points.iter().map(|p| p.z).collect()
    }
}

fn parse_input(inp: &String) -> Vec<Scanner> {
    inp.split("\n\n")
        .map(|scanner| Scanner {
            points: scanner.split("\n").skip(1).map(|point| Point::from_iter(point.split(","))).collect()
        })
    .collect()
}

fn p1(scanners: Vec<Scanner>) -> usize {
    scanners.iter().flat_map(|s| s.points.iter()
        .flat_map(move |p1| s.points.iter()
            .map(move |p2| p1.manhatan(&p2))
        )
    ).collect::<HashSet<i32>>().len()
}

fn main() {
    let inp = fs::read_to_string("input.txt").unwrap();
    let scanners = parse_input(&inp);
    println!("Part 1: {}", p1(scanners));
    // println!("Part 2: {:?}", p2(scanners));
}
