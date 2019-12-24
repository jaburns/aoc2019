
pub fn main() {
    let _tape: Vec<i64> = std::fs::read_to_string("data/day25.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    println!("Day 25");
}