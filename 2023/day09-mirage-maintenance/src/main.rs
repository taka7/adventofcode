fn is_all_same(vs: &Vec<i64>) -> bool {
    vs.windows(2).all(|w| w[0] == w[1])
}

fn solve(vv: &Vec<i64>) -> i64 {
    let mut vs = Vec::new();

    let mut latest = vv;
    while !is_all_same(&latest) {
        let next = latest.windows(2).map(|v| v[1] - v[0]).collect::<Vec<i64>>();
        vs.push(next);
        latest = &vs[vs.len() - 1];
    }

    vs.into_iter()
        .rev()
        .fold(0, |acc, vs| acc + vs[vs.len() - 1])
        + vv[vv.len() - 1]
}

fn main() {
    let nums = std::io::stdin()
        .lines()
        .map(|v| v.unwrap())
        .map(|line| {
            line.split(' ')
                .map(|word| word.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    println!("{}", nums.iter().map(|v| solve(v)).sum::<i64>());
}
