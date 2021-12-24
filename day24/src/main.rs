mod program;

use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use itertools::Itertools;
use std::cmp::Reverse;

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
        let score = program::run(&p);
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
        let score = program::run(&child);
        children.push((score, child));

        let child: Vec<u8> = p[0].1.iter().zip(p[1].1.iter()).enumerate().map(|(i, (a, b))| if i%2 == 0 {*a} else {*b}).collect();
        let score = program::run(&child);
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
                .take(9)
                .map(|x| x.clone())
                .collect()
        }
    }
    
}

fn get_mutants(reference: &[u8], count: u8) -> Vec<(i64, Vec<u8>)> {
    let values_range = Uniform::from(1..=9);
    let len_numbers = Uniform::from(0..14);
    let mut rng = thread_rng();
    (0..count).map(|_| {
        let mutation = len_numbers.sample(&mut rng);
        let mutant: Vec<u8> = reference.iter()
            .take(mutation)
            .chain(std::iter::once(&values_range.sample(&mut rng)))
            .chain(reference.iter().skip(mutation+1))
            .map(|x| *x)
            .collect();
        (program::run(&mutant), mutant)
    }).collect()
}

fn total_mutant() -> (i64, Vec<u8>) {
    let values_range = Uniform::from(1..=9);
    let mut rng = thread_rng();
    let mutant: Vec<u8> = (0..14).map(|_| values_range.sample(&mut rng)).collect();
    (program::run(&mutant), mutant)
}

fn simulation(sort: Sort) -> String {
    let mut population = initial_population();

    while population.iter().map(|x| x.0).min().unwrap() > 0 {
        population.extend(get_mutants(&population[0].1, 4));
        population.extend(get_mutants(&population.last().unwrap().1, 4));
        population.push(total_mutant());
        population = next_generation(&population, &sort);
    }

    for _ in 0..5000 {
        population.extend(get_mutants(&population[0].1, 4));
        population.extend(get_mutants(&population.last().unwrap().1, 4));
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