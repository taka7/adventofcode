use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::OnceLock;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn next(&self, x: i32, y: i32) -> Self {
        Self {
            x: (self.x as i32 + x) as usize,
            y: (self.y as i32 + y) as usize,
        }
    }
}

static OFFSETS: OnceLock<HashMap<char, Vec<(i32, i32)>>> = OnceLock::new();

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
    maze.iter()
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
        .map(|v| Pos { x: v.0, y: v.1 })
        .unwrap()
}

fn get_val(maze: &Vec<Vec<char>>, pos: &Pos) -> char {
    maze[pos.y][pos.x]
}

fn valid_next(maze: &Vec<Vec<char>>, cur: &Pos, next: &Pos) -> bool {
    OFFSETS
        .get()
        .unwrap()
        .get(&get_val(maze, &next))
        .map(|vs| vs.iter().any(|off| Pos::next(&next, off.0, off.1) == *cur))
        == Some(true)
}

fn push_candidate(maze: &Vec<Vec<char>>, pos: &Pos, visited: &mut HashSet<Pos>) -> Vec<Pos> {
    let mut next = Vec::new();

    let c = get_val(maze, pos);

    let offsets = OFFSETS.get().unwrap().get(&c).unwrap();

    offsets.iter().for_each(|off| {
        let n = Pos::next(&pos, off.0, off.1);
        if visited.contains(&n) {
            return;
        }

        if valid_next(&maze, pos, &n) {
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

fn part1(maze: &Vec<Vec<char>>, start: Pos) {
    let mut counts = make_map(&maze, 0);

    let mut visited = HashSet::new();
    let mut current = HashSet::new();
    visited.insert(start);
    current.insert(start);

    let mut count = 0;
    while current.len() != 0 {
        count += 1;
        current = current
            .iter()
            .flat_map(|c| push_candidate(&maze, &c, &mut visited))
            .collect::<HashSet<_>>();
        current.iter().for_each(|n| {
            counts[n.y][n.x] = count;
        });
    }
    println!("part1: {}", count - 1);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn print_map(wall: &Vec<Vec<Option<bool>>>) {
    println!("Map");
    for y in wall {
        for x in y {
            let c = match x {
                Some(true) => '.',
                Some(false) => '+',
                None => ' ',
            };
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

#[allow(dead_code)]
fn print_maze(maze: &Vec<Vec<char>>) {
    for y in maze {
        for x in y {
            print!("{x}");
        }
        println!("");
    }
    println!("");
}

#[allow(dead_code)]
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

fn get_wall_next(maze: &Vec<Vec<char>>, pos: &Pos, prev: Option<Pos>) -> Option<Pos> {
    OFFSETS
        .get()
        .unwrap()
        .get(&get_val(&maze, &pos))
        .map(|vs| {
            vs.iter()
                .filter_map(|v| {
                    let next = pos.next(v.0, v.1);
                    if prev != Some(next) && valid_next(&maze, &pos, &next) {
                        Some(next)
                    } else {
                        None
                    }
                })
                .next()
        })
        .flatten()
}

fn generate_wall(maze: &Vec<Vec<char>>, start: Pos) -> Vec<Vec<bool>> {
    let mut wall = make_map(maze, false);

    let mut current = HashSet::new();
    current.insert(start);

    while current.len() != 0 {
        let mut visited = HashSet::new();

        let start = current.iter().next().unwrap().clone();
        let mut next = get_wall_next(maze, &start, None);
        visited.insert(start);

        let mut prev = start;
        while next.is_some() && next != Some(start) && !visited.contains(&next.unwrap()) {
            let cur_pos = next.unwrap();
            visited.insert(cur_pos);

            next = get_wall_next(maze, &cur_pos, Some(prev)).map(|n| {
                prev = cur_pos;
                n
            });
        }

        visited.iter().for_each(|pos| {
            wall[pos.y][pos.x] = next == Some(start);
            current.remove(pos);
        });
    }

    wall
}

fn detect_char_at_start(maze: &Vec<Vec<char>>, y: usize, x: usize) -> char {
    let s = Pos { x: x, y: y };

    let valid_w = valid_next(&maze, &s, &s.next(-1, 0));
    let valid_e = valid_next(&maze, &s, &s.next(1, 0));
    let valid_n = valid_next(&maze, &s, &s.next(0, -1));
    let valid_s = valid_next(&maze, &s, &s.next(0, 1));

    if valid_w && valid_s {
        '7'
    } else if valid_s && valid_e {
        'F'
    } else if valid_e && valid_n {
        'L'
    } else if valid_n && valid_w {
        'J'
    } else {
        panic!("Unknow movable spec");
    }
}

fn generate_expanded_map(maze: &Vec<Vec<char>>, wall: &Vec<Vec<bool>>) -> Vec<Vec<Option<bool>>> {
    let ysize = maze.len() * 2 + 1;
    let xsize = maze[0].len() * 2 + 1;

    let mut line = Vec::new();
    line.resize(xsize, Some(false));
    let mut expand_maze = Vec::new();
    expand_maze.resize(ysize, line);

    wall.iter().enumerate().for_each(|(y, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, v)| **v)
            .for_each(|(x, _)| {
                let yy = y * 2 + 1;
                let xx = x * 2 + 1;
                expand_maze[yy][xx] = None;
                let tile = match maze[y][x] {
                    'S' => detect_char_at_start(maze, y, x),
                    _ => maze[y][x],
                };
                match tile {
                    'F' => {
                        expand_maze[yy + 1][xx] = None;
                        expand_maze[yy][xx + 1] = None;
                    }
                    'J' => {
                        expand_maze[yy - 1][xx] = None;
                        expand_maze[yy][xx - 1] = None;
                    }
                    'L' => {
                        expand_maze[yy - 1][xx] = None;
                        expand_maze[yy][xx + 1] = None;
                    }
                    '7' => {
                        expand_maze[yy + 1][xx] = None;
                        expand_maze[yy][xx - 1] = None;
                    }
                    '-' => {
                        expand_maze[yy][xx - 1] = None;
                        expand_maze[yy][xx + 1] = None;
                    }
                    '|' => {
                        expand_maze[yy - 1][xx] = None;
                        expand_maze[yy + 1][xx] = None;
                    }
                    _ => panic!("Unknown tile {} at ({x},{y})", maze[y][x]),
                };
            });
    });

    expand_maze
}

fn visit(map: &mut Vec<Vec<Option<bool>>>, pos: &Pos) -> Vec<Pos> {
    let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let ysize = map.len() as i32;
    let xsize = map[0].len() as i32;

    offsets
        .iter()
        .filter_map(|off| {
            let y = pos.y as i32 + off.1;
            let x = pos.x as i32 + off.0;

            if y < 0 || y >= ysize || x < 0 || x >= xsize {
                return None;
            }

            let y = y as usize;
            let x = x as usize;
            match map[y][x] {
                Some(false) => {
                    map[y][x] = Some(true);
                    Some(Pos { x: x, y: y })
                }
                _ => None,
            }
        })
        .collect()
}

fn visit_from_outside(map: &Vec<Vec<Option<bool>>>) -> Vec<Vec<Option<bool>>> {
    let mut visited_map = map.clone();

    let mut to_be_visited = HashSet::new();
    to_be_visited.insert(Pos { x: 0, y: 0 });

    while !to_be_visited.is_empty() {
        let pos = to_be_visited.iter().next().unwrap().clone();
        visit(&mut visited_map, &pos).into_iter().for_each(|p| {
            to_be_visited.insert(p);
            ()
        });

        to_be_visited.remove(&pos);
    }
    visited_map
}

fn part2(maze: &Vec<Vec<char>>, start: Pos) {
    let wall = generate_wall(&maze, start);
    let expand_map = generate_expanded_map(&maze, &wall);
    let visited_map = visit_from_outside(&expand_map);

    /*
    print_maze(&maze);
    print_wall(&wall);
    print_maze_with_wall(&maze, &wall);
    print_map(&expand_map);
    print_map(&visited_map);
    */

    let count = visited_map
        .iter()
        .enumerate()
        .filter(|(y, _)| y % 2 == 1)
        .map(|(_, line)| {
            line.iter()
                .enumerate()
                .filter(|(x, _)| x % 2 == 1)
                .filter(|(_, v)| **v == Some(false))
                .count()
        })
        .sum::<usize>();
    println!("part2: {count}");
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

    OFFSETS.set(offset_map).unwrap();

    let maze = get_maze();
    let start = find_start(&maze);
    part1(&maze, start);
    part2(&maze, start);
}
