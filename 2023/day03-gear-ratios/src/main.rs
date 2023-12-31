#[derive(Debug)]
struct Rec {
    row: usize,
    column: usize,
    width: usize,
    number: u32,
}

fn main() -> Result<(), std::io::Error> {
    let lines = std::io::stdin().lines();

    let mut table = lines
        .map(|v| format!(".{}.", v.unwrap()))
        .collect::<Vec<String>>();

    let table_width = table[0].len();
    table.insert(0, ".".repeat(table_width));
    table.push(".".repeat(table_width));

    let nums = table
        .iter()
        .map(|line| {
            let mut i = 0;
            let mut vs = Vec::new();
            while i < line.len() {
                if let Some(n) = line.get(i..).unwrap().find(|c: char| c.is_digit(10)) {
                    let end = line
                        .get((i + n)..)
                        .unwrap()
                        .find(|c: char| !c.is_digit(10))
                        .unwrap_or(line.len() - (i + n));
                    let number = line
                        .get((i + n)..(i + n + end))
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    vs.push((i + n, end, number));
                    i = i + n + end + 1;
                } else {
                    break;
                }
            }
            vs
        })
        .collect::<Vec<_>>();

    let iter = nums
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, vs)| {
            vs.into_iter().for_each(|v| {
                acc.push(Rec {
                    row: row - 1,
                    column: v.0 - 1,
                    width: v.1 + 2,
                    number: v.2,
                })
            });
            acc
        })
        .into_iter()
        .filter_map(|r| {
            if table[r.row]
                .get(r.column..r.column + r.width)
                .unwrap()
                .chars()
                .chain(
                    table[r.row + 1]
                        .get(r.column..r.column + 1)
                        .unwrap()
                        .chars(),
                )
                .chain(
                    table[r.row + 1]
                        .get(r.column + r.width - 1..r.column + r.width)
                        .unwrap()
                        .chars(),
                )
                .chain(
                    table[r.row + 2]
                        .get(r.column..r.column + r.width)
                        .unwrap()
                        .chars(),
                )
                .any(|c| c != '.')
            {
                Some(r.number)
            } else {
                None
            }
        });

    println!("{:?}", iter.sum::<u32>());

    let gears = table
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let mut vs = Vec::new();
            let mut i = 0;
            while i < line.len() {
                if let Some(c) = line.get(i..).unwrap().find('*') {
                    vs.push((row, i + c));
                    i = i + c + 1;
                } else {
                    break;
                }
            }
            vs
        })
        .fold(Vec::new(), |mut acc, v| {
            acc.extend(v);
            acc
        });

    let n = gears
        .iter()
        .map(|g| {
            (g.0 - 1..=g.0 + 1)
                .map(|r| {
                    nums[r]
                        .iter()
                        .filter_map(|num| {
                            if num.0 <= g.1 + 1 && num.0 + num.1 > g.1 - 1 {
                                Some(num.2)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .fold(Vec::new(), |mut acc, vs| {
                    vs.into_iter().for_each(|v| acc.push(v));
                    acc
                })
        })
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("{:?}", n);

    Ok(())
}
