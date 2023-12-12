fn parse() -> (Vec<(u32, u32)>, (u64, u64)) {
    let lines = std::io::stdin().lines();
    let vs = lines
        .map(|v| v.unwrap())
        .map(|line| {
            let colon = line.find(':').unwrap();
            (
                line.get(colon + 1..)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|word| !word.is_empty())
                    .map(|v| v.trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
                line.get(colon + 1..)
                    .unwrap()
                    .chars()
                    .filter(|c| *c != ' ')
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap(),
            )
        })
        .collect::<Vec<(Vec<_>, u64)>>();
    (
        vs[0]
            .0
            .iter()
            .zip(vs[1].0.iter())
            .map(|(t, d)| (*t, *d))
            .collect::<Vec<(u32, u32)>>(),
        (vs[0].1, vs[1].1),
    )
}

fn get_count(time: u64, distance: u64) -> u64 {
    let start = (0..=time)
        .map(|hold| (hold, (time - hold) * hold))
        .filter(|(_, d)| *d > distance)
        .take(1)
        .collect::<Vec<(u64, u64)>>();
    let end = (0..=time)
        .rev()
        .map(|hold| (hold, (time - hold) * hold))
        .filter(|(_, d)| *d > distance)
        .take(1)
        .collect::<Vec<(u64, u64)>>();
    end[0].0 - start[0].0 + 1
}

fn main() -> Result<(), std::io::Error> {
    let (vs1, vs2) = parse();

    println!(
        "part1: {}",
        vs1.into_iter()
            .map(|(t, d)| get_count(t as u64, d as u64))
            .product::<u64>()
    );

    println!("part2: {:?}", get_count(vs2.0, vs2.1));

    Ok(())
}
