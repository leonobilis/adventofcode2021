use std::fs;
use std::collections::HashMap;


fn parse_input(inp: String) -> (String, HashMap<String, (String, String)>) {
    let (template, rules) = match inp.split("\n\n").take(2).collect::<Vec<_>>()[..] {
        [a, b] => (a, b),
        _ => panic!()
    };
    (
        String::from(template),
        rules.lines()
            .map(|line| match line.split(" -> ").take(2).collect::<Vec<&str>>()[..] {
                [a, b] => (a.to_string(), (a[..1].to_string() + b, b.to_string() + &a[1..])),
                _ => panic!()
            })
            .collect()
    )
}

fn process(template: &str, rules: &HashMap<String, (String, String)>, iterations: u8) -> u64 {
    let mut cache = HashMap::new();

    let tasks_init: Vec<(String, u8)> = template.chars()
        .zip(template.chars().skip(1))
        .map(|(a, b)| ([a, b].iter().collect::<String>(), iterations))
        .collect();
    
    let mut tasks = tasks_init.clone();

    while !tasks.is_empty() {
        if let Some((pair, i)) = tasks.pop() {
            if i == 1 {
                let mut count_map = HashMap::new();
                count_map.insert(rules.get(&pair).unwrap().0.chars().last().unwrap(), 1u64);
                cache.insert((pair, i), count_map);
                continue;
            }
            let next = rules.get(&pair).unwrap();
            match (cache.get(&(next.0.clone(), i-1)), cache.get(&(next.1.clone(), i-1))) {
                (Some(a), Some(b)) => {
                    let mut count_map = HashMap::new();
                    count_map.insert(rules.get(&pair).unwrap().0.chars().last().unwrap(), 1u64);
                    for (chr, count) in a {
                        let e = count_map.entry(*chr).or_insert(0);
                        *e += count;
                    }
                    for (chr, count) in b {
                        let e = count_map.entry(*chr).or_insert(0);
                        *e += count;
                    }
                    cache.insert((pair, i), count_map);
                },
                (a, b) => {
                    tasks.push((pair, i));
                    if a.is_none() {
                        tasks.push((next.0.clone(), i-1));
                    }
                    if b.is_none() {
                        tasks.push((next.1.clone(), i-1))
                    }
                }
            }
        }
    }
    
    let mut sum = HashMap::new();
    for c in template.chars() {
        let e = sum.entry(c).or_insert(0);
        *e += 1;
    }
    for t in tasks_init {
        for (chr, count) in cache.get(&t).unwrap() {
            let e = sum.entry(*chr).or_insert(0);
            *e += count;
        }
    }
    sum.values().max().unwrap() - sum.values().min().unwrap()
}

fn p1(template: &str, rules: &HashMap<String, (String, String)>) -> u64 {
    process(template, rules, 10)
}

fn p2(template: &str, rules: &HashMap<String, (String, String)>) -> u64 {
    process(template, rules, 40)
}

fn main() {

    let inp = fs::read_to_string("input.txt").unwrap();
    let (template, rules) = parse_input(inp);

    println!("Part 1: {}", p1(&template, &rules));
    println!("Part 2: {}", p2(&template, &rules));

}
