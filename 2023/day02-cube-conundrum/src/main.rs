#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}
#[derive(Debug)]
struct Grab {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}
struct Game {
    id: u32,
    grab: Vec<Grab>,
}

fn parse(line: &str) -> Game {
    let p = line.find(':').unwrap();
    let game = line.get(0..p).unwrap();
    let id = game.split(' ').nth(1).unwrap().parse::<u32>().unwrap();

    let gs = line
        .get(p + 1..)
        .unwrap()
        .split(';')
        .map(|gs| {
            gs.split(',')
                .map(|g| {
                    let mut iter = g.trim().split(' ');
                    let num = iter.next().unwrap().trim().parse::<u32>().unwrap();
                    let color = iter.next().unwrap().trim();
                    match color {
                        "red" => Cube::Red(num),
                        "green" => Cube::Green(num),
                        "blue" => Cube::Blue(num),
                        _ => panic!("Unknown cube"),
                    }
                })
                .collect::<Vec<Cube>>()
        })
        .map(|cs| {
            let mut red = None;
            let mut green = None;
            let mut blue = None;
            for c in cs {
                match c {
                    Cube::Red(num) => red = Some(num),
                    Cube::Green(num) => green = Some(num),
                    Cube::Blue(num) => blue = Some(num),
                }
            }
            Grab { red, green, blue }
        })
        .collect::<Vec<Grab>>();
    Game { id: id, grab: gs }
}

fn main() -> Result<(), std::io::Error> {
    let lines = std::io::stdin().lines();

    let ids = lines
        .map(|v| v.unwrap())
        .map(|line| parse(&line))
        .collect::<Vec<Game>>();

    let part1_sum = ids
        .iter()
        .filter_map(|game| {
            if game.grab.iter().all(|gr| {
                gr.red.unwrap_or_default() <= 12
                    && gr.green.unwrap_or_default() <= 13
                    && gr.blue.unwrap_or_default() <= 14
            }) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<u32>();

    let part2_sum = ids
        .iter()
        .map(|game| {
            game.grab.iter().fold((0, 0, 0), |(ar, ag, ab), cs| {
                (
                    std::cmp::max(ar, cs.red.unwrap_or_default()),
                    std::cmp::max(ag, cs.green.unwrap_or_default()),
                    std::cmp::max(ab, cs.blue.unwrap_or_default()),
                )
            })
        })
        .map(|(r, g, b)| r * g * b)
        .sum::<u32>();

    println!("{:?}", part1_sum);
    println!("{:?}", part2_sum);
    Ok(())
}
