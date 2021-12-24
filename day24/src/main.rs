use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use itertools::Itertools;
use std::cmp::Reverse;

// from input 
fn program(inputs: &[u8]) -> i64 {

    let mut w: i64;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;

    w = *inputs.get(0).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 12;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 1;
    y *= x;
    z += y;
    w = *inputs.get(1).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 13;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 9;
    y *= x;
    z += y;
    w = *inputs.get(2).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 12;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 11;
    y *= x;
    z += y;
    w = *inputs.get(3).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -13;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 6;
    y *= x;
    z += y;
    w = *inputs.get(4).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 6;
    y *= x;
    z += y;
    w = *inputs.get(5).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 15;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 13;
    y *= x;
    z += y;
    w = *inputs.get(6).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -14;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 13;
    y *= x;
    z += y;
    w = *inputs.get(7).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 12;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 5;
    y *= x;
    z += y;
    w = *inputs.get(8).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -8;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 7;
    y *= x;
    z += y;
    w = *inputs.get(9).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 14;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 2;
    y *= x;
    z += y;
    w = *inputs.get(10).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -9;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 10;
    y *= x;
    z += y;
    w = *inputs.get(11).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -11;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 14;
    y *= x;
    z += y;
    w = *inputs.get(12).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -6;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 7;
    y *= x;
    z += y;
    w = *inputs.get(13).unwrap() as i64;
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -5;
    x = if x == w {1} else {0};
    x = if x == 0 {1} else {0};
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 1;
    y *= x;
    z += y;

    z
}

fn initial_population() -> Vec<(i64, Vec<u8>)> {
    let mut rng = thread_rng();

    let mut values: Vec<Vec<u8>> = (0..14).map(|_| {
        let mut v: Vec<u8> = (1..=9).collect();
        v.shuffle(&mut rng);
        v
    }).collect();

    (1..=9).map(move |_| {
        let mut p = Vec::new();
        for v in values.iter_mut() {
            p.push(v.pop().unwrap());
        }
        let score = program(&p);
        (score, p)
    }).collect()
}

enum Sort {
    Asc,
    Desc
}

fn next_generation(population: &Vec<(i64, Vec<u8>)>, sort: &Sort) -> Vec<(i64, Vec<u8>)> {
    let new_populaton: Vec<(i64, Vec<u8>)> = population.iter().permutations(2).flat_map(|p| {
        let mut children = Vec::new();

        let child: Vec<u8> = p[0].1.iter().take(7).chain(p[1].1.iter().skip(7)).map(|x| *x).collect();

        let score = program(&child);
        children.push((score, child));

        let child: Vec<u8> = p[0].1.iter().zip(p[1].1.iter()).enumerate().map(|(i, (a, b))| if i%2 == 0 {*a} else {*b}).collect();
        
        let score = program(&child);
        children.push((score, child));

        children
    }).collect();

    match sort {
        Sort::Asc => {
            new_populaton.iter()
                .unique()
                .sorted_by_key(|(score,child)| (score, child))
                .take(9)
                .map(|x| x.clone())
                .collect()

        },
        Sort::Desc => {
            new_populaton.iter()
                .unique()
                .sorted_by_key(|(score,child)| (score, Reverse(child)))
                // .rev()
                .take(9)
                .map(|x| x.clone())
                .collect()
        }
    }
    
}

fn simulation(sort: Sort) -> String {
    let values_range = Uniform::from(1..=9);
    let len_numbers = Uniform::from(0..14);

    let mut rng = thread_rng();

    let mut population = initial_population();

    while population.iter().map(|x| x.0).max().unwrap() > 0 {
        
        for _ in 0..4 {
            let mutation = len_numbers.sample(&mut rng);
            let mutant: Vec<u8> = population[0].1.iter().enumerate().map(|(i, c)| if i == mutation {values_range.sample(&mut rng)} else {*c}).collect();
            population.push((program(&mutant), mutant));
        }

        let total_mutant: Vec<u8> = (0..14).map(|_| values_range.sample(&mut rng)).collect();
        population.push((program(&total_mutant), total_mutant));

        population = next_generation(&population, &sort);
    }

    for _ in 0..10000 {
        for _ in 0..4 {
            let mutation = len_numbers.sample(&mut rng);
            let mutant: Vec<u8> = population[0].1.iter().enumerate().map(|(i, c)| if i == mutation {values_range.sample(&mut rng)} else {*c}).collect();
            population.push((program(&mutant), mutant));
        }

        population = next_generation(&population, &sort);
    }
    
    String::from_iter(population[0].1.iter().map(|x| x.to_string()))
}

fn p1() -> String {
    simulation(Sort::Desc)
}

fn p2() -> String {
    simulation(Sort::Asc)
}


fn main() {
    println!("Part 1: {}", p1());
    println!("Part 2: {}", p2());
}