#[derive(Debug)]
struct MapEntry {
    src: u32,
    dst: u32,
    len: u32,
}

type Seeds = Vec<u32>;

fn parse() -> (Seeds, Vec<Vec<MapEntry>>) {
    let lines = std::io::stdin().lines();

    let mut seeds = Vec::new();
    let (mut map, v) = lines
        .map(|v| v.unwrap())
        .filter(|line| line.len() != 0)
        .fold((Vec::new(), Vec::new()), |(mut m, mut v), line| {
            if let Some(colon) = line.find(':') {
                if line.get(0..colon - 1).unwrap() == "seed" {
                    seeds = line
                        .get(colon + 1..)
                        .unwrap()
                        .trim()
                        .split(' ')
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect::<Vec<_>>();
                } else {
                    if v.len() != 0 {
                        m.push(v);
                        v = Vec::new();
                    }
                }
            } else {
                let vs = line
                    .split(' ')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                v.push(MapEntry {
                    src: vs[1],
                    dst: vs[0],
                    len: vs[2],
                });
            }
            (m, v)
        });
    if v.len() != 0 {
        map.push(v);
    }
    (seeds, map)
}

fn main() -> Result<(), std::io::Error> {
    let (seeds, maps) = parse();

    let fs = seeds.iter().map(|s| {
        let mut ss = *s;
        for m in &maps {
            let vs = m
                .iter()
                .filter_map(|m| {
                    if m.src <= ss && ss < m.src + m.len {
                        Some(ss - m.src + m.dst)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if vs.len() != 0 {
                ss = vs[0];
            }
        }
        ss
    });

    println!("part-1: {}", fs.min().unwrap());

    Ok(())
}
