#[derive(Debug)]
struct MapEntry {
    src: u64,
    dst: u64,
    len: u64,
}

type Seeds = Vec<u64>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct SeedRange {
    start: u64,
    len: u64,
}

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
                        .map(|v| v.parse::<u64>().unwrap())
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
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
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

fn get_seeds_range(seeds: &Seeds) -> Vec<SeedRange> {
    seeds
        .chunks(2)
        .map(|vs| SeedRange {
            start: vs[0],
            len: vs[1],
        })
        .collect()
}

impl SeedRange {
    fn apply_map(&self, map: &MapEntry) -> (Vec<SeedRange>, Vec<SeedRange>) {
        let end = self.start + self.len;
        let mend = map.src + map.len;

        let mut mapped = Vec::new();
        let mut unmapped = Vec::new();

        if self.start < map.src {
            if end > map.src {
                unmapped.push(SeedRange {
                    start: self.start,
                    len: map.src - self.start,
                });
                mapped.push(SeedRange {
                    start: map.dst,
                    len: core::cmp::min(end, mend) - map.src,
                });
                if end > mend {
                    unmapped.push(SeedRange {
                        start: mend,
                        len: end - mend,
                    });
                }
            } else {
                unmapped.push(*self);
            }
        } else if self.start >= map.src && self.start < mend {
            mapped.push(SeedRange {
                start: map.dst + self.start - map.src,
                len: core::cmp::min(end, mend) - self.start,
            });
            if end > mend {
                unmapped.push(SeedRange {
                    start: mend,
                    len: end - mend,
                });
            }
        } else {
            unmapped.push(*self);
        }
        (mapped, unmapped)
    }

    fn apply_map_entries(&self, maps: &Vec<MapEntry>) -> Vec<SeedRange> {
        let (mut mapped, unmapped) = maps.iter().fold(
            (Vec::new(), vec![*self]),
            |(mut mapped, pre_unmapped), map| {
                let unmapped = pre_unmapped.iter().fold(vec![], |mut acc, seed| {
                    let (m, u) = seed.apply_map(map);
                    mapped.extend(m);
                    acc.extend(u);
                    acc
                });
                (mapped, unmapped)
            },
        );

        mapped.extend(unmapped);
        mapped
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_left() {
        assert_eq!(
            (vec![], vec![SeedRange { start: 10, len: 5 }]),
            SeedRange { start: 10, len: 5 }.apply_map(&MapEntry {
                src: 15,
                len: 4,
                dst: 100,
            })
        );
        assert_eq!(
            (
                vec![SeedRange { start: 100, len: 1 }],
                vec![SeedRange { start: 10, len: 4 }],
            ),
            SeedRange { start: 10, len: 5 }.apply_map(&MapEntry {
                src: 14,
                len: 4,
                dst: 100,
            })
        );
        assert_eq!(
            (
                vec![SeedRange { start: 100, len: 4 }],
                vec![
                    SeedRange { start: 10, len: 4 },
                    SeedRange { start: 18, len: 2 }
                ],
            ),
            SeedRange { start: 10, len: 10 }.apply_map(&MapEntry {
                src: 14,
                len: 4,
                dst: 100,
            })
        );
    }

    #[test]
    fn test_middle() {
        assert_eq!(
            (vec![SeedRange { start: 100, len: 3 }], vec![]),
            SeedRange { start: 14, len: 3 }.apply_map(&MapEntry {
                src: 14,
                len: 4,
                dst: 100,
            })
        );
        assert_eq!(
            (vec![SeedRange { start: 101, len: 3 }], vec![]),
            SeedRange { start: 15, len: 3 }.apply_map(&MapEntry {
                src: 14,
                len: 4,
                dst: 100,
            })
        );
        assert_eq!(
            (
                vec![SeedRange { start: 101, len: 3 }],
                vec![SeedRange { start: 18, len: 2 }],
            ),
            SeedRange { start: 15, len: 5 }.apply_map(&MapEntry {
                src: 14,
                len: 4,
                dst: 100,
            })
        );
    }

    #[test]
    fn test_right() {
        assert_eq!(
            (vec![], vec![SeedRange { start: 18, len: 3 }]),
            SeedRange { start: 18, len: 3 }.apply_map(&MapEntry {
                src: 14,
                len: 4,
                dst: 100,
            })
        );
    }

    #[test]
    fn test_get_mapped() {
        let seed = SeedRange { start: 10, len: 10 };
        let maps = vec![
            MapEntry {
                src: 12,
                len: 1,
                dst: 112,
            },
            MapEntry {
                src: 15,
                len: 2,
                dst: 115,
            },
            MapEntry {
                src: 19,
                len: 1,
                dst: 119,
            },
        ];

        let mut result = seed.apply_map_entries(&maps);
        result.sort();
        assert_eq!(
            vec![
                SeedRange { start: 10, len: 2 },
                SeedRange { start: 13, len: 2 },
                SeedRange { start: 17, len: 2 },
                SeedRange { start: 112, len: 1 },
                SeedRange { start: 115, len: 2 },
                SeedRange { start: 119, len: 1 },
            ],
            result,
        );
    }
}

fn find_lowest(seeds: &Vec<SeedRange>, maps: &Vec<Vec<MapEntry>>) -> u64 {
    seeds
        .iter()
        .map(|seed_range| {
            maps.iter().fold(vec![*seed_range], |acc, map| {
                acc.iter()
                    .map(|s| s.apply_map_entries(map))
                    .flatten()
                    .collect()
            })
        })
        .flatten()
        .min()
        .unwrap()
        .start
}

fn main() -> Result<(), std::io::Error> {
    let (seeds, maps) = parse();
    let seeds1 = seeds
        .iter()
        .map(|v| SeedRange { start: *v, len: 1 })
        .collect::<Vec<SeedRange>>();
    let seeds2 = get_seeds_range(&seeds);

    let part1 = find_lowest(&seeds1, &maps);
    let part2 = find_lowest(&seeds2, &maps);

    println!("part-1: {}", part1);
    println!("part-2: {}", part2);

    Ok(())
}
