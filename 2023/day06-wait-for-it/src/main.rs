fn parse() -> Vec<(u32, u32)> {
    let lines = std::io::stdin().lines();
    let vs = lines
        .map(|v| v.unwrap())
        .map(|line| {
            let colon = line.find(':').unwrap();
            line.get(colon + 1..)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|word| !word.is_empty())
                .map(|v| v.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    vs[0]
        .iter()
        .zip(vs[1].iter())
        .map(|(t, d)| (*t, *d))
        .collect::<Vec<(u32, u32)>>()
}

fn goal_candidates(time: u32, distance: u32) -> Vec<(u32, u32)> {
    (0..=time)
        .map(|hold| (hold, (time - hold) * hold))
        .filter(|(_, d)| *d > distance)
        .collect::<Vec<(u32, u32)>>()
}

fn main() -> Result<(), std::io::Error> {
    let vs = parse();

    println!(
        "part1: {}",
        vs.into_iter()
            .map(|(t, d)| goal_candidates(t, d).len())
            .product::<usize>()
    );

    Ok(())
}
