use num::integer::*;

fn count_visible_asteroids(map: &Vec<Vec<bool>>, x: i32, y: i32) -> u32 {
    let mut count = 0u32;

    for iy in 0..(map.len() as i32) {
        for ix in 0..(map[0].len() as i32) {
            if !map[iy as usize][ix as usize] { continue };
            if ix == x && iy == y { continue };

            let mut dx = x - ix;
            let mut dy = y - iy;
            let gcd = dx.gcd(&dy);
            dx /= gcd;
            dy /= gcd;

            let mut sx = ix;
            let mut sy = iy;
            loop {
                sx += dx;
                sy += dy;

                if sx == x && sy == y {
                    count += 1;
                    break;
                }

                if map[sy as usize][sx as usize] { break };
            }
        }
    }

    count
}

fn find_best_asteroid(map: &Vec<Vec<bool>>) -> (i32, i32, u32) {
    let mut result = (0, 0, 0);

    for iy in 0..(map.len() as i32) {
        for ix in 0..(map[0].len() as i32) {
            let count = count_visible_asteroids(map, ix, iy);
            if count > result.2 {
                result = (ix, iy, count);
            }
        }
    }

    result
}

pub fn main() {
    let map: Vec<Vec<bool>> = std::fs::read_to_string("data/day10.txt").unwrap()
        .lines()
        .map(|x| x.trim().chars().map(|x| x == '#').collect())
        .collect();

    let (_x, _y, result0) = find_best_asteroid(&map);

    println!("{} {}", result0, result0);
}