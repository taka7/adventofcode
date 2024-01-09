use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn next(pos: &Pos, x: i32, y: i32) -> Pos {
        Pos {
            x: (pos.x as i32 + x) as usize,
            y: (pos.y as i32 + y) as usize,
        }
    }
}

#[allow(dead_code)]
fn print_matrix(vs: &Vec<Vec<char>>) {
    for line in vs {
        println!("{}", String::from_iter(line));
    }
}
#[allow(dead_code)]
fn print_counts(vs: &Vec<Vec<i32>>) {
    for line in vs {
        for l in line {
            print!("{l}");
        }
        println!("");
    }
}

fn get_maze() -> Vec<Vec<char>> {
    let maze = std::io::stdin()
        .lines()
        .map(|v| v.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let column = maze[0].len();

    let mut maze = maze
        .into_iter()
        .map(|mut v| {
            v.insert(0, '.');
            v.push('.');
            v
        })
        .collect::<Vec<Vec<char>>>();

    let mut new_row = Vec::new();
    new_row.resize(column + 2, '.');
    maze.insert(0, new_row.clone());
    maze.push(new_row);

    maze
}

fn find_start(maze: &Vec<Vec<char>>) -> Pos {
    let v = maze
        .iter()
        .enumerate()
        .filter_map(|(row, vs)| {
            if let Some(c) = vs
                .iter()
                .enumerate()
                .filter_map(|(column, c)| if *c == 'S' { Some(column) } else { None })
                .next()
            {
                Some((c, row))
            } else {
                None
            }
        })
        .next()
        .unwrap();
    Pos { x: v.0, y: v.1 }
}

fn get_val(maze: &Vec<Vec<char>>, pos: &Pos) -> char {
    maze[pos.y][pos.x]
}

fn valid_next(
    maze: &Vec<Vec<char>>,
    offset_map: &HashMap<char, Vec<(i32, i32)>>,
    cur: &Pos,
    next: &Pos,
) -> bool {
    offset_map
        .get(&get_val(maze, &next))
        .map(|vs| vs.iter().any(|off| Pos::next(&next, off.0, off.1) == *cur))
        == Some(true)
}

fn push_candidate(
    maze: &Vec<Vec<char>>,
    pos: &Pos,
    offset_map: &HashMap<char, Vec<(i32, i32)>>,
    visited: &mut HashSet<Pos>,
) -> Vec<Pos> {
    let mut next = Vec::new();

    let c = get_val(maze, pos);

    println!("c: {}", c);
    let offsets = offset_map.get(&c).unwrap();

    offsets.iter().for_each(|off| {
        let n = Pos::next(&pos, off.0, off.1);
        if visited.contains(&n) {
            return;
        }

        if valid_next(&maze, offset_map, pos, &n) {
            next.push(n);
            visited.insert(n);
        }
    });

    next
}

fn make_map<T>(maze: &Vec<Vec<char>>, val: T) -> Vec<Vec<T>>
where
    T: Default + Copy,
{
    maze.iter().fold(Vec::new(), |mut acc, c| {
        let mut vs = Vec::new();
        vs.resize(c.len(), val);
        acc.push(vs);
        acc
    })
}

fn part1(maze: &Vec<Vec<char>>, offset_map: &HashMap<char, Vec<(i32, i32)>>) {
    let mut counts = make_map(&maze, 0);

    let start = find_start(&maze);

    let mut visited = HashSet::new();
    let mut current = HashSet::new();
    visited.insert(start);
    current.insert(start);

    let mut count = 0;
    while current.len() != 0 {
        count += 1;
        current = current
            .iter()
            .flat_map(|c| push_candidate(&maze, &c, offset_map, &mut visited))
            .collect::<HashSet<_>>();
        current.iter().for_each(|n| {
            counts[n.y][n.x] = count;
        });
    }
    println!("part1: {}", count - 1);
}

fn print_wall(wall: &Vec<Vec<bool>>) {
    println!("Wall");
    for y in wall {
        for x in y {
            let c = match x {
                true => 'T',
                false => ' ',
            };
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

fn print_maze(maze: &Vec<Vec<char>>) {
    for y in maze {
        for x in y {
            print!("{x}");
        }
        println!("");
    }
    println!("");
}

fn print_maze_with_wall(maze: &Vec<Vec<char>>, wall: &Vec<Vec<bool>>) {
    for (y, ys) in maze.iter().enumerate() {
        for (x, v) in ys.iter().enumerate() {
            let c = if wall[y][x] == true { *v } else { ' ' };
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

fn part2(maze: &Vec<Vec<char>>, offset_map: &HashMap<char, Vec<(i32, i32)>>) {
    let mut maze = maze.clone();
    //    let s = find_start(&maze);
    //    maze[s.y][s.x] = 'L';

    let mut wall = make_map(&maze, false);

    let mut current = (1..(maze.len() - 1))
        .flat_map(|r| (1..(maze[0].len() - 1)).map(move |c| Pos { x: c, y: r }))
        .collect::<HashSet<Pos>>();

    print_maze(&maze);

    while current.len() != 0 {
        let mut visited = HashSet::new();

        let mut loop_closed = false;
        let mut pos = current.iter().next().unwrap().clone();
        while !visited.contains(&pos) {
            visited.insert(pos);

            let mut next = None;
            if let Some(vs) = offset_map.get(&get_val(&maze, &pos)) {
                let next1 = Pos::next(&pos, vs[0].0, vs[0].1);
                let next2 = Pos::next(&pos, vs[1].0, vs[1].1);
                let visited1 = visited.contains(&next1);
                let visited2 = visited.contains(&next2);
                if !visited1 {
                    next = Some(next1);
                } else if !visited2 {
                    next = Some(next2);
                } else {
                    loop_closed = valid_next(&maze, &offset_map, &pos, &next1)
                        && valid_next(&maze, &offset_map, &pos, &next2);
                }
            }

            if let Some(n) = next {
                if valid_next(&maze, &offset_map, &pos, &n) {
                    pos = n;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        visited.iter().for_each(|pos| {
            wall[pos.y][pos.x] = loop_closed;
            current.remove(pos);
        });
    }

    print_wall(&wall);
    print_maze_with_wall(&maze, &wall);
}

fn main() {
    let mut offset_map = HashMap::new();
    offset_map.insert('S', vec![(0, -1), (-1, 0), (1, 0), (0, 1)]);
    offset_map.insert('F', vec![(1, 0), (0, 1)]);
    offset_map.insert('J', vec![(0, -1), (-1, 0)]);
    offset_map.insert('L', vec![(0, -1), (1, 0)]);
    offset_map.insert('7', vec![(-1, 0), (0, 1)]);
    offset_map.insert('|', vec![(0, -1), (0, 1)]);
    offset_map.insert('-', vec![(-1, 0), (1, 0)]);

    let maze = get_maze();

    //    part1(&maze, &offset_map);
    part2(&maze, &offset_map);
}
