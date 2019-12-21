use crate::expanse::Expanse;

#[derive(Clone, Debug, Eq, PartialEq)]
enum TileKind {
    Path,
    Portal(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Tile {
    pub dist: u32,
    pub kind: TileKind,
}

fn is_capital_letter(ch: char) -> bool {
    ch >= 'A' && ch <= 'Z'
}

fn load_maze(chars: &Vec<Vec<char>>) -> Expanse<Tile> {
    let mut result = Expanse::new();

    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            if chars[y][x] != '.' {
                continue;
            }

            let kind = if is_capital_letter(chars[y][x - 1]) {
                TileKind::Portal(format!("{}{}", chars[y][x - 2], chars[y][x - 1]))
            } else if is_capital_letter(chars[y][x + 1]) {
                TileKind::Portal(format!("{}{}", chars[y][x + 1], chars[y][x + 2]))
            } else if is_capital_letter(chars[y - 1][x]) {
                TileKind::Portal(format!("{}{}", chars[y - 2][x], chars[y - 1][x]))
            } else if is_capital_letter(chars[y + 1][x]) {
                TileKind::Portal(format!("{}{}", chars[y + 1][x], chars[y + 2][x]))
            } else {
                TileKind::Path
            };

            result.write(
                x as i32,
                y as i32,
                Tile {
                    dist: 0,
                    kind: kind,
                },
            );
        }
    }

    result
}

fn find_portals(maze: &Expanse<Tile>, id: &str) -> Vec<(i32, i32)> {
    maze.find_many(|Tile { kind, .. }| {
        if let TileKind::Portal(x) = kind {
            x == id
        } else {
            false
        }
    })
}

fn can_walk(maze: &Expanse<Tile>, x: i32, y: i32) -> bool {
    match maze.read(x, y) {
        Some(Tile { dist, .. }) => *dist == 0,
        _ => false,
    }
}

fn can_portal(maze: &Expanse<Tile>, x: i32, y: i32) -> Option<(i32, i32)> {
    if let Some(Tile { kind: TileKind::Portal(id), .. }) = maze.read(x, y) {
        for &(px, py) in find_portals(maze, id).iter() {
            if px != x && py != y && maze.read(px, py).unwrap().dist == 0 {
                return Some((px, py));
            }
        }
        None
    } else {
        None
    }
}

fn solve_flat_maze(maze: &Expanse<Tile>) -> u32 {
    let mut maze = maze.clone();

    let (x, y) = find_portals(&maze, "AA")[0];
    
    let start_tile = maze.at(x, y).unwrap();
    start_tile.kind = TileKind::Path;

    let mut frontier = vec![ (x, y) ];
    let mut cur_dist = 1;

    while frontier.len() > 0 {
        for i in 0..frontier.len() {
            let (x, y) = frontier[i];
            maze.at(x, y).unwrap().dist = cur_dist;
        }

        let mut new_frontier = Vec::<(i32, i32)>::new();

        for i in 0..frontier.len() {
            let (x, y) = frontier[i];
            if can_walk(&maze, x + 1, y) { new_frontier.push((x + 1, y)); }
            if can_walk(&maze, x - 1, y) { new_frontier.push((x - 1, y)); }
            if can_walk(&maze, x, y + 1) { new_frontier.push((x, y + 1)); }
            if can_walk(&maze, x, y - 1) { new_frontier.push((x, y - 1)); }
            if let Some(new_pos) = can_portal(&maze, x, y) { new_frontier.push(new_pos); }
        }

        cur_dist += 1;
        frontier = new_frontier;
    }

    let (zx, zy) = find_portals(&maze, "ZZ")[0];
    maze.read(zx, zy).unwrap().dist - 1
}

fn solve_recursive_maze(maze: &Expanse<Tile>) -> u32 {
    0
}

pub fn main() {
    let map_chars: Vec<Vec<char>> = std::fs::read_to_string("data/day20.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let maze = load_maze(&map_chars);
    let result0 = solve_flat_maze(&maze);
    let result1 = solve_recursive_maze(&maze);

    println!("{} {}", result0, result1);
}
