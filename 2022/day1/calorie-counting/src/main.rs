use itertools::Itertools;

fn main() -> Result<(), std::io::Error> {
    let lines = std::io::stdin().lines();

    let m = lines
        .into_iter()
        .map(|v| v.unwrap())
        .group_by(|elt| !elt.is_empty())
        .into_iter()
        .filter_map(|(k, v)| if k { Some(v) } else { None })
        .map(|v| {
            v.map(|elt| elt.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| v.iter().sum::<i32>())
        .collect::<Vec<i32>>()
        .into_iter()
        .sorted_by(|a, b| b.cmp(a));

    let mut vs = m.take(3);
    let best = vs.next().unwrap();
    println!("The best is {}", best);
    println!("The sum of top 3 is {}", best + vs.sum::<i32>());

    Ok(())
}
