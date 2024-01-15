use itertools::Itertools;

fn get_cosmic() -> Vec<Vec<char>> {
    std::io::stdin()
        .lines()
        .map(|v| v.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_expansion(cosmic: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let expand_columns = cosmic
        .iter()
        .enumerate()
        .filter_map(|(c, line)| {
            if line.iter().all(|v| *v == '.') {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let mut expand_rows = Vec::new();
    for column in 0..cosmic[0].len() {
        if cosmic
            .iter()
            .flat_map(|line| {
                line.iter()
                    .enumerate()
                    .filter_map(|(c, v)| if c == column { Some(v) } else { None })
            })
            .all(|v| *v == '.')
        {
            expand_rows.push(column);
        };
    }

    (expand_rows, expand_columns)
}

fn get_galaxies(cosmic: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    cosmic
        .iter()
        .enumerate()
        .flat_map(|(r, lines)| {
            lines.iter().enumerate().filter_map(
                move |(c, v)| {
                    if *v == '#' {
                        Some((c, r))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

struct Pos<Idx> {
    x: Idx,
    y: Idx,
}

fn calc_length(
    g1: Pos<usize>,
    g2: Pos<usize>,
    rows: &Vec<usize>,
    columns: &Vec<usize>,
    distance_ratio: usize,
) -> usize {
    let (min_row, max_row) = if g1.x < g2.x {
        (g1.x, g2.x)
    } else {
        (g2.x, g1.x)
    };
    let skips = rows
        .iter()
        .filter(|r| min_row < **r && **r < max_row)
        .count();

    let dist_row = max_row - min_row - skips + skips * distance_ratio;

    let (min_column, max_column) = if g1.y < g2.y {
        (g1.y, g2.y)
    } else {
        (g2.y, g1.y)
    };
    let skips = columns
        .iter()
        .filter(|c| min_column < **c && **c < max_column)
        .count();
    let dist_column = max_column - min_column - skips + skips * distance_ratio;

    dist_row + dist_column
}

fn calc_total_distance(cosmic: &Vec<Vec<char>>, distance_ratio: usize) -> usize {
    let (exp_columns, exp_rows) = get_expansion(cosmic);
    let galaxies = get_galaxies(cosmic);

    let pairs = galaxies.iter().combinations(2);
    pairs
        .map(|p| {
            calc_length(
                Pos {
                    x: p[0].0,
                    y: p[0].1,
                },
                Pos {
                    x: p[1].0,
                    y: p[1].1,
                },
                &exp_columns,
                &exp_rows,
                distance_ratio,
            )
        })
        .sum::<usize>()
}

fn part1(cosmic: &Vec<Vec<char>>) {
    let total_len = calc_total_distance(cosmic, 2);
    println!("part1: {total_len}");
}

fn part2(cosmic: &Vec<Vec<char>>) {
    let total_len = calc_total_distance(cosmic, 1_000_000);
    println!("part2: {total_len}");
}

fn part2_10(cosmic: &Vec<Vec<char>>) {
    let total_len = calc_total_distance(cosmic, 10);
    println!("part2_10: {total_len}");
}

fn part2_100(cosmic: &Vec<Vec<char>>) {
    let total_len = calc_total_distance(cosmic, 100);
    println!("part2_100: {total_len}");
}

fn main() {
    let cosmic = get_cosmic();
    part1(&cosmic);
    part2(&cosmic);

    part2_10(&cosmic);
    part2_100(&cosmic);
}
