use std::collections::HashMap;

#[derive(Debug)]
struct Set {
    hand: [char; 5],
    bid: u32,
}

fn parse() -> Vec<Set> {
    let lines = std::io::stdin().lines();
    lines
        .map(|v| v.unwrap())
        .map(|line| {
            let mut iter = line.split(' ');
            let hand = iter
                .next()
                .unwrap()
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let bid = iter.next().unwrap().parse::<u32>().unwrap();
            Set {
                hand: hand,
                bid: bid,
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Type {
    Five,
    Four,
    FullHouse,
    Three,
    Two,
    One,
    High,
}

fn get_type(hand: &[char; 5]) -> Type {
    let mut hash_map = HashMap::new();
    hand.iter().for_each(|c| {
        if let Some(x) = hash_map.get_mut(&c) {
            *x += 1;
        } else {
            hash_map.insert(c, 1);
        }
    });

    let mut vs = hash_map.values().collect::<Vec<&i32>>();
    vs.sort_by(|a, b| b.partial_cmp(a).unwrap());
    if vs.len() == 1 {
        Type::Five
    } else if vs.len() == 2 {
        // Four or FullHouse
        if *vs[0] == 4 {
            Type::Four
        } else {
            Type::FullHouse
        }
    } else if *vs[0] == 3 {
        Type::Three
    } else if *vs[0] == 2 {
        // One or Two
        if *vs[1] != 1 {
            Type::Two
        } else {
            Type::One
        }
    } else {
        Type::High
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_type() {
        assert_eq!(Type::Five, get_type(&['v', 'v', 'v', 'v', 'v']));
        assert_eq!(Type::Four, get_type(&['1', 'v', 'v', 'v', 'v']));
        assert_eq!(Type::FullHouse, get_type(&['1', 'v', 'v', '1', 'v']));
        assert_eq!(Type::Three, get_type(&['2', 'v', 'v', '1', 'v']));
        assert_eq!(Type::Two, get_type(&['2', 'v', '2', '1', 'v']));
        assert_eq!(Type::One, get_type(&['2', 'v', '2', '1', 'k']));
        assert_eq!(Type::High, get_type(&['2', 'v', 'q', '1', 'k']));
    }
}

fn main() -> Result<(), std::io::Error> {
    let cards = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let strength_map: HashMap<char, usize> =
        HashMap::from_iter(cards.iter().enumerate().map(|(i, c)| (*c, i)));

    let sets = parse();

    let mut vs: Vec<_> = sets.iter().map(|set| (get_type(&set.hand), set)).collect();
    vs.sort_by(|(t1, set1), (t2, set2)| {
        let v = t1.partial_cmp(t2).unwrap();
        if v.is_eq() {
            set1.hand
                .iter()
                .zip(set2.hand.iter())
                .map(|(s1, s2)| {
                    strength_map
                        .get(s1)
                        .unwrap()
                        .partial_cmp(strength_map.get(s2).unwrap())
                        .unwrap()
                })
                .find(|v| v.is_ne())
                .unwrap()
        } else {
            v
        }
    });
    let sum = vs
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, set))| (rank + 1) * set.bid as usize)
        .sum::<usize>();
    println!("part1: {:?}", sum);

    Ok(())
}
