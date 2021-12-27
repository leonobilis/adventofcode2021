use std::fs;
use itertools::Itertools;

enum CheckResult {
    ExplodeLeft,
    ExplodeRight,
    SplitLeft,
    SplitRight,
    SplitExplodeLeft,
    SplitExplodeRight,
    CheckLeftRight
}

#[derive(Clone)]
struct Pair {
    left: PairValue,
    right: PairValue
}

impl Pair {
    fn reduce (&mut self) {
        while let Some(_) = self.explosion_check(0) {};
        while let Some(_) = self.split_check(0) {};   
    }

    fn explosion_check(&mut self, i: u8) -> Option<(Option<u32>, Option<u32>)> {
        let check_result = match (i, &self.left, &self.right) {
            (3, PairValue::Pair(_), _) => CheckResult::ExplodeLeft,
            (3, _, PairValue::Pair(_)) => CheckResult::ExplodeRight,
            _ => CheckResult::CheckLeftRight
        };

        match check_result {
            CheckResult::ExplodeLeft => {
                let l = self.left.is_pair_ref().unwrap();
                let (lnum, rnum);
                if let (Some(ll), Some(rr)) = (l.left.is_number_ref(), l.right.is_number_ref()) {
                    lnum = *ll;
                    rnum = *rr;
                } else {
                    panic!("check error");
                }
                
                self.left = PairValue::Number(0);
                return Some((Some(lnum), self.right.try_add_left(rnum)));
                
            },
            CheckResult::ExplodeRight => {
                let r = self.right.is_pair_ref().unwrap();
                let (lnum, rnum);
                if let (Some(ll), Some(rr)) = (r.left.is_number_ref(), r.right.is_number_ref()) {
                    lnum = *ll;
                    rnum = *rr;
                    
                } else {
                    panic!("check error");
                }

                self.right = PairValue::Number(0);
                return Some((self.left.try_add_right(lnum), Some(rnum)));
            },
            CheckResult::CheckLeftRight=> {
                if let Some((ll, rr)) = self.left.explosion_check(i+1) {
                    let mut rr = rr;
                    if let Some(rrr) = rr {
                        rr = self.right.try_add_left(rrr);
                    }
                    return Some((ll, rr));
                }
                
                if let Some((ll, rr)) = self.right.explosion_check(i+1) {
                    let mut ll = ll;
                    if let Some(lll) = ll {
                        ll = self.left.try_add_right(lll);
                    }
                    return Some((ll, rr));
                }
                return None;
            },
            _ => None
        }
    }

    fn split_check(&mut self, i: u8) -> Option<(Option<u32>, Option<u32>)> {
        let check_result = match (i, &self.left, &self.right) {
            (3, PairValue::Number(l), _) if *l >= 10 => CheckResult::SplitExplodeLeft,
            (3, _, PairValue::Number(r)) if *r >= 10 => CheckResult::SplitExplodeRight,
            (_, PairValue::Number(l), _) if *l >= 10 => CheckResult::SplitLeft,
            (_, _, PairValue::Number(r)) if *r >= 10 => CheckResult::SplitRight,
            _ => CheckResult::CheckLeftRight
        };

        match check_result {
            CheckResult::SplitExplodeLeft => {
                let lnum = self.left.is_number_ref().unwrap().clone();
                self.left = PairValue::Number(0);
                return Some((Some(lnum/2), self.right.try_add_left((lnum as f32/2.0).ceil() as u32)));
            },
            CheckResult::SplitLeft => {
                let lnum = self.left.is_number_ref().unwrap();
                self.left = PairValue::Pair(Box::new(Pair{
                    left: PairValue::Number(lnum/2),
                    right: PairValue::Number((*lnum as f32/2.0).ceil() as u32)
                }));
                return Some((None, None));
            },
            CheckResult::SplitExplodeRight => {
                let rnum = self.right.is_number_ref().unwrap().clone();
                self.right = PairValue::Number(0);
                return Some((self.left.try_add_right(rnum/2), Some((rnum as f32/2.0).ceil() as u32)));
            },
            CheckResult::SplitRight => {
                let rnum = self.right.is_number_ref().unwrap();
                self.right = PairValue::Pair(Box::new(Pair{
                    left: PairValue::Number(rnum/2),
                    right: PairValue::Number((*rnum as f32/2.0).ceil() as u32)
                }));
                return Some((None, None));
            },
            CheckResult::CheckLeftRight=> {
                if let Some((ll, rr)) = self.left.split_check(i+1) {
                    let mut rr = rr;
                    if let Some(rrr) = rr {
                        rr = self.right.try_add_left(rrr);
                    }
                    return Some((ll, rr));
                }
                
                if let Some((ll, rr)) = self.right.split_check(i+1) {
                    let mut ll = ll;
                    if let Some(lll) = ll {
                        ll = self.left.try_add_right(lll);
                    }
                    return Some((ll, rr));
                }
                return None;
            },
            _ => None
        }
    }
}

#[derive(Clone)]
enum PairValue {
    Number(u32),
    Pair(Box<Pair>)
}

impl PairValue {
    fn is_pair(self) -> Option<Box<Pair>>{
        match self {
            PairValue::Pair(pair) => Some(pair),
            PairValue::Number(_) => None
        }
    }

    fn is_pair_ref(&mut self) -> Option<&Box<Pair>>{
        match self {
            PairValue::Pair(pair) => Some(pair),
            PairValue::Number(_) => None
        }
    }

    fn is_number_ref(&self) -> Option<&u32>{
        match self {
            PairValue::Number(number) => Some(number),
            PairValue::Pair(_) => None
        }
    }

    fn try_add_left(&mut self, value: u32) -> Option<u32> {
        match self {
            PairValue::Number(x) => {
                *self = PairValue::Number(*x+value);
                None
            }
            PairValue::Pair(pair) => {
                if pair.left.try_add_left(value).is_none() || pair.right.try_add_left(value).is_none() {
                    return None;
                }
                Some(value)
            }
        }
    }

    fn try_add_right(&mut self, value: u32) -> Option<u32> {
        match self {
            PairValue::Number(x) => {
                *self = PairValue::Number(*x+value);
                None
            }
            PairValue::Pair(pair) => {
                if pair.right.try_add_right(value).is_none() || pair.left.try_add_right(value).is_none() {
                    return None;
                }
                Some(value)
            }
        }
    }

    fn explosion_check(&mut self, i: u8) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            PairValue::Pair(p) => p.explosion_check(i),
            PairValue::Number(_) => None
        }
    }

    fn split_check(&mut self, i: u8) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            PairValue::Pair(p) => p.split_check(i),
            PairValue::Number(_) => None
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            PairValue::Pair(p) => 3*p.left.magnitude() + 2*p.right.magnitude(),
            PairValue::Number(n) => *n
        }
    }
}

fn parse_input(inp: &String) -> Vec<Box<Pair>> {
    fn parse_pair_value(chars: &mut std::str::Chars) -> PairValue {
        match chars.next() {
            Some('[') => PairValue::Pair(parse_pair(chars)),
            Some(c) => PairValue::Number(c.to_digit(10).unwrap()),
            None => panic!("expected '[' or number")
        }
    }
    fn parse_pair(chars: &mut std::str::Chars) -> Box<Pair> {
        let left = parse_pair_value(chars);
        match chars.next() {
            Some(',') => (),
            _ => panic!("expected ','")
        };
        let right = parse_pair_value(chars);
        match chars.next() {
            Some(']') => (),
            _ => panic!("expected ']'")
        };
        Box::new(Pair{right, left})
    }

    inp.lines().filter_map(|l| parse_pair_value(&mut l.chars()).is_pair()).rev().collect()
}

fn p1(mut numbers: Vec<Box<Pair>>) -> u32 {
    let mut add = PairValue::Pair(numbers.pop().unwrap());
    while let Some(number) = numbers.pop() {
        let r = PairValue::Pair(number);
        let mut a = Box::new(Pair{left: add, right: r});
        a.reduce();
        add = PairValue::Pair(a);
    }
    add.magnitude()
}

fn p2(numbers: Vec<Box<Pair>>) -> Vec<u32> {
    numbers.iter().rev().permutations(2).map(|v| {
        let a = PairValue::Pair(v[0].clone());
        let b = PairValue::Pair(v[1].clone());
        let mut add = Box::new(Pair{left: a, right: b});
        add.reduce();
        PairValue::Pair(add).magnitude()
    }).sorted().rev().take(5).collect() // Why?
}

fn main() {

    let inp = fs::read_to_string("input.txt").unwrap();
    let numbers = parse_input(&inp);
    println!("Part 1: {}", p1(numbers.clone()));
    println!("Part 2: {:?}", p2(numbers));

}
