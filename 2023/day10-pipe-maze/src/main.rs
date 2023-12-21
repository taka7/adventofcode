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

fn push_candidate(
    maze: &Vec<Vec<char>>,
    pos: &Pos,
    offset_map: &HashMap<char, Vec<(i32, i32)>>,
    visited: &mut HashSet<Pos>,
) -> Vec<Pos> {
    let mut next = Vec::new();

    let c = get_val(maze, pos);

    let offsets = offset_map.get(&c).unwrap();

    offsets.iter().for_each(|off| {
        let n = Pos::next(&pos, off.0, off.1);
        if visited.contains(&n) {
            return;
        }

        let c = get_val(maze, &n);
        if offset_map
            .get(&c)
            .map(|vs| vs.iter().any(|off| Pos::next(&n, off.0, off.1) == *pos))
            == Some(true)
        {
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

    part1(&maze, &offset_map);
}
