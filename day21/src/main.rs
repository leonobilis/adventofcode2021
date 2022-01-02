use std::cmp::{max, Eq, PartialEq};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Player {
    pos: u32,
    score: u64,
}

fn p1(player1: &u32, player2: &u32) -> u32 {
    let mut dice = (1..=100).cycle();
    let mut player1 = Player {
        pos: *player1,
        score: 0,
    };
    let mut player2 = Player {
        pos: *player2,
        score: 0,
    };
    let mut r = 0;

    let mut calc_position = |x: u32| {
        (x + [dice.next(), dice.next(), dice.next()]
            .iter()
            .map(|d| d.unwrap())
            .sum::<u32>()
            - 1)
            % 10
            + 1
    };
    while player2.score < 1000 {
        player1.pos = calc_position(player1.pos);
        player1.score += player1.pos as u64;
        r += 3;
        if player1.score >= 1000 {
            return player2.score as u32 * r;
        }
        player2.pos = calc_position(player2.pos);
        player2.score += player2.pos as u64;
        r += 3;
    }
    player1.score as u32 * r
}

fn p2(player1: &u32, player2: &u32) -> u64 {
    let r_count: HashMap<u8, u8> = [(3, 1), (9, 1), (4, 3), (8, 3), (5, 6), (7, 6), (6, 7)]
        .iter()
        .cloned()
        .collect();

    let mut cache = HashMap::new();
    let tasks_init: Vec<_> = r_count
        .keys()
        .map(|roll_sum| {
            (
                Player {
                    pos: *player1,
                    score: 0,
                },
                Player {
                    pos: *player2,
                    score: 0,
                },
                *roll_sum as u8,
            )
        })
        .collect();

    let mut tasks = tasks_init.clone();

    while let Some((player_a, player_b, roll_sum)) = tasks.pop() {
        let count = *r_count.get(&roll_sum).unwrap() as u64;
        let new_pos = (player_a.pos + roll_sum as u32 - 1) % 10 + 1;
        let new_player_a = Player {
            pos: new_pos,
            score: player_a.score + new_pos as u64,
        };
        if new_player_a.score >= 21 {
            cache.insert((player_a, player_b, roll_sum), (count, 0u64));
            continue;
        }

        let (universes, missing): (Vec<_>, Vec<_>) = r_count
            .keys()
            .map(|roll| {
                (
                    roll,
                    cache.get(&(player_b.clone(), new_player_a.clone(), *roll)),
                )
            })
            .partition(|(_, cached)| cached.is_some());
        if universes.len() == 7 {
            let (lose_sum, win_sum) = universes
                .iter()
                .map(|(_, cached)| *cached.unwrap())
                .reduce(|acc, cached| (acc.0 + cached.0, acc.1 + cached.1))
                .unwrap();

            cache.insert(
                (player_a, player_b, roll_sum),
                (win_sum * count, lose_sum * count),
            );
        } else {
            tasks.push((player_a, player_b.clone(), roll_sum));
            missing
                .iter()
                .for_each(|(roll, _)| tasks.push((player_b.clone(), new_player_a.clone(), **roll)));
        }
    }

    let (win, lose) = tasks_init
        .iter()
        .map(|task| *cache.get(task).unwrap())
        .reduce(|acc, cached| (acc.0 + cached.0, acc.1 + cached.1))
        .unwrap();

    max(win, lose)
}

fn main() {
    // input
    let player1 = 8;
    let player2 = 5;

    println!("Part 1: {}", p1(&player1, &player2));
    println!("Part 2: {}", p2(&player1, &player2));
}
