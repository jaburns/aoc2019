mod day1;
mod day2;
mod day3;

fn run_from_arg(arg: i32) {
    match arg {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        _ => {}
    }
}

fn run_default() {
    day3::day3()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        run_from_arg(args[1].parse::<i32>().unwrap())
    } else {
        run_default()
    }
}