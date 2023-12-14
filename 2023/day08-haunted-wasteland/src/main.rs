use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Marker {
    Start,
    End,
    No,
}

#[derive(Debug, Clone)]
struct Key {
    id: String,
    marker: Marker,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Key {}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
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

fn part2(seq: &Vec<Direction>, maps_orig: &HashMap<String, Choice>) -> usize {
    let maps: HashMap<Key, Choice> = HashMap::from_iter(maps_orig.iter().map(|(k, v)| {
        let marker = match k.chars().nth(k.len() - 1).unwrap() {
            'A' => Marker::Start,
            'Z' => Marker::End,
            _ => Marker::No,
        };
        (
            Key {
                id: k.clone(),
                marker: marker,
            },
            v.clone(),
        )
    }));

    let mut poses = vec![maps
        .keys()
        .filter_map(|k| {
            if k.marker == Marker::Start {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect::<Vec<Key>>()];
    println!("poses: {:?}", poses);
    while poses[poses.len() - 1]
        .iter()
        .any(|k| k.marker != Marker::End)
    {
        match seq.iter().try_fold(poses, |mut acc, dir| {
            //            println!("Dir: {:?}", dir);
            if acc[acc.len() - 1].iter().any(|k| k.marker != Marker::End) {
                let iter = acc[acc.len() - 1].iter().map(|pos| {
                    let choice = maps.get(pos).unwrap();
                    let next = match dir {
                        Direction::Left => &choice.left,
                        Direction::Right => &choice.right,
                    };
                    //                    println!("pos: {:?} next:{:?}", pos, next);
                    Key {
                        id: next.clone(),
                        marker: if next.chars().nth(next.len() - 1) == Some('Z') {
                            Marker::End
                        } else {
                            Marker::No
                        },
                    }
                });
                acc.push(iter.collect::<Vec<Key>>());
            }
            Some(acc)
        }) {
            None => panic!("panic"),
            Some(x) => poses = x,
        }
        //        println!("poses: {:?}, len:{}", poses, poses.len());
    }

    poses.len() - 1
}

fn main() -> Result<(), std::io::Error> {
    let (seq, maps) = parse();

    //    println!("{:?}", part1(&seq, &maps));
    println!("{:?}", part2(&seq, &maps));

    Ok(())
}
