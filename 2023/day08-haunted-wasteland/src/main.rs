use num::Zero;
use std::cmp::Eq;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::ops::Rem;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Choice {
    left: String,
    right: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Marker {
    Start,
    End,
    No,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Key {
    id: String,
}

fn parse() -> (Vec<Direction>, HashMap<String, Choice>) {
    let mut lines = std::io::stdin().lines().map(|v| v.unwrap());

    let seqs = lines.next().unwrap();
    let seqs = seqs
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        })
        .collect::<Vec<Direction>>();

    let lines = lines.skip_while(|x| x.trim().len() == 0);

    let v = lines.map(|line| {
        let mut iter = line.split('=').enumerate().flat_map(|(i, v)| -> Vec<&str> {
            let v = v.trim();
            if i == 0 {
                vec![v]
            } else {
                let pos = v[1..v.len() - 1].find(',').unwrap();
                let left = v[1..1 + pos].trim();
                let right = v[1 + pos + 1..v.len() - 1].trim();
                vec![left, right]
            }
        });
        (
            iter.next().map(|s| s.to_string()).unwrap(),
            Choice {
                left: iter.next().map(|s| s.to_string()).unwrap(),
                right: iter.next().map(|s| s.to_string()).unwrap(),
            },
        )
    });

    let hash_map = HashMap::from_iter(v);
    (seqs, hash_map)
}

fn part1(seq: &Vec<Direction>, maps: &HashMap<String, Choice>) -> usize {
    let mut pos = "AAA";
    let mut count = 0;
    while pos != "ZZZ" {
        count += seq
            .iter()
            .map(|dir| {
                let choice = maps.get(pos).unwrap();
                let next = match dir {
                    Direction::Left => &choice.left,
                    Direction::Right => &choice.right,
                };
                pos = next;
                next
            })
            .take_while(|v| *v != "ZZZ")
            .count();
    }
    count + 1
}

fn gcd<I>(a1: I, a2: I) -> I
where
    I: Rem<Output = I> + PartialOrd + Copy + Zero,
{
    let (mut a1, mut a2) = if a1 > a2 { (a1, a2) } else { (a2, a1) };
    while !(a1 % a2).is_zero() {
        a1 = a1 % a2;
        (a1, a2) = if a1 > a2 { (a1, a2) } else { (a2, a1) }
    }
    a2
}

fn gcd_from(seq: &Vec<i32>) -> i32 {
    seq.iter().skip(1).fold(seq[0], |acc, s| gcd(acc, *s))
}

fn gcd_from_iter<'a, I, U>(mut seq: I) -> U
where
    I: Iterator<Item = U>,
    U: Rem<Output = U> + PartialOrd + Copy + Zero,
{
    let first = seq.next().unwrap();
    seq.fold(first, |acc, s| gcd(acc, s))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(1, gcd(2, 3));
        assert_eq!(2, gcd(2, 8));
        assert_eq!(5, gcd(105, 235));
        assert_eq!(15, gcd(105, 225));
    }

    #[test]
    fn test_gcd_from() {
        assert_eq!(1, gcd_from(&vec![2, 3]));
        assert_eq!(2, gcd_from(&vec![2, 8]));
        assert_eq!(5, gcd_from(&vec![105, 235]));
        assert_eq!(15, gcd_from(&vec![105, 225]));
        assert_eq!(15, gcd_from(&vec![105, 225, 45]));
        assert_eq!(1, gcd_from(&vec![105, 225, 44]));
        assert_eq!(3, gcd_from(&vec![105, 225, 3]));
    }
}

fn part2(seq: &Vec<Direction>, maps_orig: &HashMap<String, Choice>) -> usize {
    let maps: HashMap<String, (Marker, Choice)> =
        HashMap::from_iter(maps_orig.iter().map(|(k, v)| {
            let marker = match k.chars().nth(k.len() - 1).unwrap() {
                'A' => Marker::Start,
                'Z' => Marker::End,
                _ => Marker::No,
            };
            (k.clone(), (marker, v.clone()))
        }));

    let starts = maps
        .iter()
        .filter_map(|(k, v)| if v.0 == Marker::Start { Some(k) } else { None });

    let counts = starts.map(|k| {
        seq.iter()
            .cycle()
            .scan(k, |acc, s| {
                let v = maps.get(*acc).unwrap();
                let next = match s {
                    Direction::Left => &v.1.left,
                    Direction::Right => &v.1.right,
                };
                if v.0 == Marker::End {
                    None
                } else {
                    *acc = next;
                    Some(())
                }
            })
            .count()
    });

    let gcd = gcd_from_iter(counts.clone());
    counts.map(|v| v / gcd).product::<usize>() * gcd
}

fn main() -> Result<(), std::io::Error> {
    let (seq, maps) = parse();

    println!("part1: {:?}", part1(&seq, &maps));
    println!("part2: {:?}", part2(&seq, &maps));

    Ok(())
}
