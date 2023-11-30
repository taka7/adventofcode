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
        .max();

    println!("{}", m.unwrap());

    Ok(())
}
