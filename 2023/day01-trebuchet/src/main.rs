use std::collections::HashMap;

fn convert(line: &str) -> String {
    let conv_table = HashMap::from([
        ("one", ('1', 2)),
        ("two", ('2', 2)),
        ("three", ('3', 4)),
        ("four", ('4', 4)),
        ("five", ('5', 3)),
        ("six", ('6', 3)),
        ("seven", ('7', 4)),
        ("eight", ('8', 4)),
        ("nine", ('9', 3)),
    ]);

    let mut i = 0;
    let mut result = String::new();
    while i < line.len() {
        let mut replace = false;
        for (p, r) in &conv_table {
            if line.get(i..).unwrap().starts_with(p) {
                result.push(r.0);
                i += r.1;
                replace = true;
                break;
            }
        }
        if !replace {
            result.push(line.chars().nth(i).unwrap());
            i += 1;
        }
    }
    result
}

fn extract_digits(line: &str) -> Vec<u32> {
    line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>()
}

fn duplicate_one<T: Copy>(mut vs: Vec<T>) -> Option<Vec<T>> {
    if vs.len() == 0 {
        return None;
    }
    if vs.len() == 1 {
        vs.push(vs[0]);
    }
    Some(vs)
}

fn gen_number(vs: &Option<Vec<u32>>) -> u32 {
    if let Some(vs) = vs {
        vs[0] * 10 + vs[vs.len() - 1]
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(convert("12345"), "12345");
        assert_eq!(convert("one2345"), "12345");
        assert_eq!(convert("1two345"), "12345");
        assert_eq!(convert("1twone"), "121");
        assert_eq!(convert("eightwone"), "821");
    }
}

fn main() -> Result<(), std::io::Error> {
    let lines = std::io::stdin().lines();

    let m = lines
        .map(|v| v.unwrap())
        .map(|line| (convert(&line), line))
        .map(|(line1, line2)| (extract_digits(&line1), extract_digits(&line2), line2))
        .map(|(vs1, vs2, line)| (duplicate_one(vs1), duplicate_one(vs2), line))
        .map(|(vs1, vs2, line)| (gen_number(&vs1), gen_number(&vs2), line));

    // for (v1, v2, line) in m.into_iter() {
    //     println!("{:?} -> {:?}  {:?}", line, v1, v2);
    // }
    let (s2, s1, _) = m.fold((0, 0, ""), |(acc1, acc2, acc3), (x1, x2, _)| {
        (acc1 + x1, acc2 + x2, acc3)
    });
    println!("The sum for part1 is {}, part2 is {}", s1, s2);
    Ok(())
}
