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
        .enumerate()
        .fold(
            Vec::<(usize, usize, usize, u32)>::new(),
            |mut acc, (row, vs)| {
                vs.into_iter().for_each(|v| acc.push((row, v.0, v.1, v.2)));
                acc
            },
        );

    let iter = nums
        .into_iter()
        .map(|(row, column, width, number)| Rec {
            row: row - 1,
            column: column - 1,
            width: width + 2,
            number: number,
        })
        .collect::<Vec<Rec>>()
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
    Ok(())
}
