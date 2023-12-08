#[derive(Debug)]
struct Card {
    id: u32,
    win: Vec<u32>,
    number: Vec<u32>,
}

fn parse(line: &str) -> Card {
    let p = line.find(':').unwrap();
    let card = line.get(4..p).unwrap();
    let id = card.trim().parse::<u32>().unwrap();

    let mut gs = line
        .get(p + 1..)
        .unwrap()
        .split('|')
        .map(|v| {
            v.trim()
                .split(' ')
                .filter_map(|n| {
                    if n.len() == 0 {
                        None
                    } else {
                        Some(n.parse::<u32>().unwrap())
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();
    Card {
        id: id,
        win: gs.remove(0),
        number: gs.remove(0),
    }
}

fn main() -> Result<(), std::io::Error> {
    let lines = std::io::stdin().lines();

    let s = lines
        .map(|v| v.unwrap())
        .map(|line| parse(&line))
        .map(|card| {
            card.number
                .iter()
                .filter(|n| card.win.iter().any(|w| w == *n))
                .count()
        })
        .collect::<Vec<usize>>()
        .into_iter()
        .filter_map(|n| {
            if n >= 1 {
                Some(2u32.pow((n - 1) as u32))
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("{:?}", s);
    Ok(())
}
