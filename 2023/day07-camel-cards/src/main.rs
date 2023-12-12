use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
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

const J: char = 'J';

fn get_hand_ordered_vec(hand: &[char; 5]) -> Vec<(char, i32)> {
    let mut hash_map = HashMap::new();
    hand.iter().for_each(|c| {
        if let Some(x) = hash_map.get_mut(&c) {
            *x += 1;
        } else {
            hash_map.insert(c, 1);
        }
    });
    let mut vs = hash_map
        .into_iter()
        .map(|(k, v)| (*k, v))
        .collect::<Vec<(char, i32)>>();
    vs.sort_by(|(_, v1), (_, v2)| v2.partial_cmp(v1).unwrap());
    vs
}

fn get_type(hand: &[char; 5]) -> Type {
    let vs = get_hand_ordered_vec(hand);
    if vs.len() == 1 {
        Type::Five
    } else if vs.len() == 2 {
        // Four or FullHouse
        if vs[0].1 == 4 {
            Type::Four
        } else {
            Type::FullHouse
        }
    } else if vs[0].1 == 3 {
        Type::Three
    } else if vs[0].1 == 2 {
        // One or Two
        if vs[1].1 != 1 {
            Type::Two
        } else {
            Type::One
        }
    } else {
        Type::High
    }
}

fn get_type_joker(hand: &[char; 5]) -> Type {
    let vs = get_hand_ordered_vec(hand);

    if vs.len() == 1 {
        Type::Five
    } else if vs.len() == 2 {
        if vs.iter().any(|(k, _)| *k == J) {
            Type::Five
        } else if vs[0].1 == 4 {
            Type::Four
        } else {
            Type::FullHouse
        }
    } else if vs[0].1 == 3 {
        if vs.iter().any(|(k, _)| *k == J) {
            Type::Four
        } else {
            Type::Three
        }
    } else if vs[0].1 == 2 {
        if vs[1].1 != 1 {
            // 2 2 1
            if vs[0].0 == J || vs[1].0 == J {
                Type::Four
            } else if vs[2].0 == J {
                Type::FullHouse
            } else {
                Type::Two
            }
        } else {
            // 2 1 1 1
            if vs.iter().any(|(k, _)| *k == J) {
                Type::Three
            } else {
                Type::One
            }
        }
    } else {
        if vs.iter().any(|(k, _)| *k == J) {
            Type::One
        } else {
            Type::High
        }
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

    #[test]
    fn test_get_type_joker() {
        assert_eq!(Type::Five, get_type_joker(&['v', 'v', 'v', 'v', 'v']));
        assert_eq!(Type::Five, get_type_joker(&[J, J, J, J, J]));

        assert_eq!(Type::Four, get_type_joker(&['1', 'v', 'v', 'v', 'v']));
        assert_eq!(Type::Five, get_type_joker(&['1', J, J, J, J]));
        assert_eq!(Type::Five, get_type_joker(&[J, 'v', 'v', 'v', 'v']));

        assert_eq!(Type::FullHouse, get_type_joker(&['1', 'v', 'v', '1', 'v']));
        assert_eq!(Type::Five, get_type_joker(&[J, 'v', 'v', J, 'v']));
        assert_eq!(Type::Five, get_type_joker(&['1', J, J, '1', J]));

        assert_eq!(Type::Three, get_type_joker(&['2', 'v', 'v', '1', 'v']));
        assert_eq!(Type::Four, get_type_joker(&['2', J, J, '1', J]));
        assert_eq!(Type::Four, get_type_joker(&['2', 'v', 'v', J, 'v']));
        assert_eq!(Type::Four, get_type_joker(&[J, 'v', 'v', '1', 'v']));

        assert_eq!(Type::Two, get_type_joker(&['2', 'v', '2', '1', 'v']));
        assert_eq!(Type::Four, get_type_joker(&['2', J, '2', '1', J]));
        assert_eq!(Type::Four, get_type_joker(&[J, 'v', J, '1', 'v']));
        assert_eq!(Type::FullHouse, get_type_joker(&['2', 'v', '2', J, 'v']));

        assert_eq!(Type::One, get_type_joker(&['2', 'v', '2', '1', 'k']));
        assert_eq!(Type::Three, get_type_joker(&[J, 'v', J, '1', 'k']));
        assert_eq!(Type::Three, get_type_joker(&['2', J, '2', '1', 'k']));
        assert_eq!(Type::Three, get_type_joker(&['2', 'v', '2', J, 'k']));
        assert_eq!(Type::Three, get_type_joker(&['2', 'v', '2', '1', J]));

        assert_eq!(Type::High, get_type_joker(&['2', 'v', 'q', '1', 'k']));
        assert_eq!(Type::One, get_type_joker(&[J, 'v', 'q', '1', 'k']));
        assert_eq!(Type::One, get_type_joker(&['2', J, 'q', '1', 'k']));
        assert_eq!(Type::One, get_type_joker(&['2', 'v', J, '1', 'k']));
        assert_eq!(Type::One, get_type_joker(&['2', 'v', 'q', J, 'k']));
        assert_eq!(Type::One, get_type_joker(&['2', 'v', 'q', '1', J]));
    }
}

fn sort(sets: &mut Vec<(Type, Set)>, map: &HashMap<char, usize>) {
    sets.sort_by(|(t1, set1), (t2, set2)| {
        let v = t1.partial_cmp(t2).unwrap();
        if v.is_eq() {
            set1.hand
                .iter()
                .zip(set2.hand.iter())
                .map(|(s1, s2)| {
                    map.get(s1)
                        .unwrap()
                        .partial_cmp(map.get(s2).unwrap())
                        .unwrap()
                })
                .find(|v| v.is_ne())
                .unwrap()
        } else {
            v
        }
    });
}

fn calc_sum(sets: &Vec<(Type, Set)>) -> usize {
    sets.iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, set))| (rank + 1) * set.bid as usize)
        .sum::<usize>()
}

fn main() -> Result<(), std::io::Error> {
    let sets = parse();

    const CARDS: [char; 13] = [
        'A', 'K', 'Q', J, 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let strength_map: HashMap<char, usize> =
        HashMap::from_iter(CARDS.iter().enumerate().map(|(i, c)| (*c, i)));
    let mut vs: Vec<_> = sets.iter().map(|set| (get_type(&set.hand), *set)).collect();
    sort(&mut vs, &strength_map);
    println!("part1: {:?}", calc_sum(&vs));

    const CARDS_J: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', J,
    ];
    let strength_map_joker: HashMap<char, usize> =
        HashMap::from_iter(CARDS_J.iter().enumerate().map(|(i, c)| (*c, i)));
    let mut vs: Vec<_> = sets
        .iter()
        .map(|set| (get_type_joker(&set.hand), *set))
        .collect();
    sort(&mut vs, &strength_map_joker);
    println!("part2: {:?}", calc_sum(&vs));

    Ok(())
}
