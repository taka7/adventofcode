use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Choice {
    left: String,
    right: String,
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

fn main() -> Result<(), std::io::Error> {
    let (seq, maps) = parse();

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
    count += 1;

    println!("{:?}", count);

    Ok(())
}
