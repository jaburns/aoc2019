use crate::expanse::Expanse;
use crate::intcode::vm::IntCodeMachine;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn move_point(&self, pos: (i32, i32)) -> (i32, i32) {
        let mut result = pos;
        match self {
            Direction::North => result.1 -= 1,
            Direction::South => result.1 += 1,
            Direction::West => result.0 -= 1,
            Direction::East => result.0 += 1,
        };
        result
    }

    pub fn relative_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn relative_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

type Scaffold = Expanse<()>;

fn load_scaffold_and_robot_pos_from_camera_view(camera_view: &str) -> (Scaffold, (i32, i32)) {
    let mut scaffold = Scaffold::new();
    let mut ix = 0i32;
    let mut iy = 0i32;

    let mut robot_pos = (0i32, 0i32);

    for c in camera_view.chars() {
        match c {
            '\n' => {
                iy += 1;
                ix = 0;
                continue;
            }
            '#' => scaffold.write(ix, iy, ()),
            '^' => robot_pos = (ix, iy),
            _ => (),
        };
        ix += 1;
    }

    (scaffold, robot_pos)
}

fn sum_scaffold_alignment_parameters(scaffold: &Scaffold) -> u32 {
    let mut sum = 0i32;

    for x in scaffold.x_range() {
        for y in scaffold.y_range() {
            if scaffold.read(x, y).is_some()
                && scaffold.read(x + 1, y).is_some()
                && scaffold.read(x - 1, y).is_some()
                && scaffold.read(x, y + 1).is_some()
                && scaffold.read(x, y - 1).is_some()
            {
                sum += x * y;
            }
        }
    }

    sum as u32
}

fn get_uncompressed_robot_path(scaffold: &Scaffold, robot_pos: (i32, i32)) -> String {
    let mut path = String::from("L,");
    let mut pos = robot_pos;
    let mut dir = Direction::West;
    let mut walk_count = 0u32;

    loop {
        let next_pos = dir.move_point(pos);
        if scaffold.read(next_pos.0, next_pos.1).is_some() {
            pos = next_pos;
            walk_count += 1;
        } else {
            let look = dir.relative_left().move_point(pos);
            if scaffold.read(look.0, look.1).is_some() {
                dir = dir.relative_left();
                path.push_str(walk_count.to_string().as_str());
                path.push_str(",L,");
                walk_count = 0;
                continue;
            }

            let look = dir.relative_right().move_point(pos);
            if scaffold.read(look.0, look.1).is_some() {
                dir = dir.relative_right();
                path.push_str(walk_count.to_string().as_str());
                path.push_str(",R,");
                walk_count = 0;
                continue;
            }

            path.push_str(walk_count.to_string().as_str());
            break;
        }
    }

    path
}

fn vacuum_and_report_dust(tape: &[i64], scaffold: &Scaffold, robot_pos: (i32, i32)) -> u32 {
    let mut machine = IntCodeMachine::new(tape);
    machine.poke(0, 2);

    let path = get_uncompressed_robot_path(scaffold, robot_pos);
    println!("{}", path);

    // "L,10,L,12,R,6,R,10,L,4,L,4,L,12,L,10,L,12,R,6,R,10,L,4,L,4,L,12,L,10,"

    0
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day17.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let camera_view: String = IntCodeMachine::run_all(&tape, &[])
        .iter()
        .map(|x| *x as u8 as char)
        .collect();

    let (scaffold, robot_pos) = load_scaffold_and_robot_pos_from_camera_view(&camera_view);

    println!("{}", scaffold.render_to_string(false, "  ", |_| " #"));

    let result0 = sum_scaffold_alignment_parameters(&scaffold);
    let result1 = vacuum_and_report_dust(&tape, &scaffold, robot_pos);

    println!("{} {}", result0, result1);
}
