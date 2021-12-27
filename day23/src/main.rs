use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord};
use std::hash::Hash;
use itertools::Itertools;


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum AmphipodaType {
    Amber,
    Bronze,
    Copper,
    Desert
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Amphipoda {
    t: AmphipodaType,
    dest: i8,
    settled: bool,
    energy: u32
}

impl Amphipoda {
    fn new(c: &char) -> Self {
        match c {
            'A' => Amphipoda{
                t: AmphipodaType::Amber,
                dest: 2,
                settled: false,
                energy: 1
            },
            'B' => Amphipoda{
                t: AmphipodaType::Bronze,
                dest: 4,
                settled: false,
                energy: 10
            },
            'C' => Amphipoda{
                t: AmphipodaType::Copper,
                dest: 6,
                settled: false,
                energy: 100
            },
            'D' => Amphipoda{
                t: AmphipodaType::Desert,
                dest: 8,
                settled: false,
                energy: 1000
            },
            _ => panic!("Unknown amphipoda type")
        }
    }
}

fn parse_input(inp: &String) -> HashMap<(i8, i8), Amphipoda> {
    inp.lines()
        .skip(2)
        .take(2)
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .filter(|c| c.is_alphabetic())
            .enumerate()
            .map(move |(x, c)| ((2 + 2*x as i8, y as i8+1), Amphipoda::new(&c)))
        ).collect()
}

#[derive(Clone)]
struct Diagram {
    amphipods: HashMap<(i8, i8), Amphipoda>,
    room_depth: i8,
    energy: u32
}

impl Diagram {
    fn new(mut amphipods: HashMap<(i8, i8), Amphipoda>, room_depth: i8) -> Self {
        for ((x, y), a) in amphipods.iter_mut() {
            if *x == a.dest && *y == room_depth {
                a.settled = true;
            }
        }
        
        Diagram {
            amphipods,
            room_depth,
            energy: 0
        }
    }

    fn move_options(&self, filter_min: u32) -> Vec<Diagram> {
        self.amphipods.iter()
            .filter(|(_, a)| !a.settled)
            .flat_map(|((x, y), a)| {
                let mut diagrams = Vec::new();
                let spaces: Vec<(i8, i8)> = if *y != 0 && !self.amphipods.contains_key(&(*x, *y-1)) {
                    (0..*x).rev().take_while(|xx| !self.amphipods.contains_key(&(*xx, 0)))
                        .map(|xx| (xx, 0))
                        .chain((*x+1..=10).take_while(|xx| !self.amphipods.contains_key(&(*xx, 0)))
                            .map(|xx| (xx, 0))
                        ).collect()
                } else if *y == 0 {
                    if *x > a.dest {
                        (a.dest..*x).rev()
                            .take_while(|xx| !self.amphipods.contains_key(&(*xx, 0)))
                            .map(|xx| (xx, 0)).collect()
                    } else {
                        (*x+1..=a.dest)
                            .take_while(|xx| !self.amphipods.contains_key(&(*xx, 0)))
                            .map(|xx| (xx, 0)).collect()
                    }
                } else {
                    Vec::new()
                };
                if spaces.contains(&(a.dest, 0)) {
                    let room: Vec<&Amphipoda> = (1..=self.room_depth).filter_map(|yy| self.amphipods.get(&(a.dest, yy))).collect();
                    if room.iter().all(|aa| aa.t == a.t) {
                        let new_pos = (a.dest, self.room_depth - room.len() as i8);
                        let new_energy = self.energy + ((x - new_pos.0).abs() + y + new_pos.1) as u32 * a.energy;
                        if new_energy < filter_min {
                            let mut diagram = self.clone();
                            let mut amp = diagram.amphipods.remove(&(*x, *y)).unwrap();
                            amp.settled = true;
                            diagram.amphipods.insert(new_pos, amp);
                            diagram.energy = new_energy;
                            diagrams.push(diagram);
                        }
                    }
                }
                if *y != 0 {
                    diagrams.extend(
                        spaces.iter().filter(|(xx, _)| *xx != 2 && *xx != 4 && *xx != 6 && *xx != 8)
                            .filter_map(|(xx, _)| {
                                let new_energy = self.energy + ((x - xx).abs() + y) as u32  * a.energy;
                                if new_energy >= filter_min {
                                    None
                                } else {
                                    let mut diagram = self.clone();
                                    let amp = diagram.amphipods.remove(&(*x, *y)).unwrap();
                                    diagram.amphipods.insert((*xx, 0), amp);
                                    diagram.energy = new_energy;
                                    Some(diagram)
                                }
                            })
                    );
                }
                diagrams
            }).collect()
    }

}

#[derive(PartialEq, Eq, Hash)]
struct CacheKey(Vec<(i8, i8, AmphipodaType)>);

fn solve(amphipods: HashMap<(i8, i8), Amphipoda>, room_depth: i8) -> u32 {
    let mut diagrams = Vec::new();
    diagrams.push(Diagram::new(amphipods.clone(), room_depth));

    let mut min = u32::MAX;
    let mut cache: HashMap<CacheKey, u32> = HashMap::new();

    while let Some(diagram) = diagrams.pop() {
        let opt_mov = diagram.move_options(min);
        for o in opt_mov {
            let entry = cache.entry(CacheKey(o.amphipods.iter().map(|((x, y), a)| (*x, *y, a.t.clone())).sorted().collect()));
            if let Entry::Occupied(mut oe) = entry {
                if *oe.get() > o.energy {
                    *oe.get_mut() = o.energy;
                } else {
                    continue;
                }
            } else if let Entry::Vacant(ve) = entry {
                ve.insert(o.energy);
            }
            if o.energy < min {
                if o.amphipods.values().all(|a| a.settled) {
                    min = o.energy;                
                }
                else {
                    diagrams.push(o);
                }
            }
        }
    }
    min
}

fn p1(amphipods: &HashMap<(i8, i8), Amphipoda>) -> u32 {
    solve(amphipods.clone(), 2)
}

fn p2(amphipods: &HashMap<(i8, i8), Amphipoda>) -> u32 {
    let mut amphipods: HashMap<(i8, i8), Amphipoda> = amphipods.iter()
        .map(|((x, y), a)| ((*x, if *y == 2 {4} else {*y}), a.clone()))
        .collect();
    amphipods.extend([
        ((2, 2), Amphipoda::new(&'D')), ((4, 2), Amphipoda::new(&'C')),
        ((6, 2), Amphipoda::new(&'B')), ((8, 2), Amphipoda::new(&'A')),
        ((2, 3), Amphipoda::new(&'D')), ((4, 3), Amphipoda::new(&'B')),
        ((6, 3), Amphipoda::new(&'A')), ((8, 3), Amphipoda::new(&'C'))
    ]);

    solve(amphipods.clone(), 4)
}

fn main() {
    let inp = fs::read_to_string("input.txt").unwrap();
    let amphipods = parse_input(&inp);
    println!("Part 1: {}", p1(&amphipods));
    println!("Part 2: {}", p2(&amphipods));
}