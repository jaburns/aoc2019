use crate::intcode::vm::{IntCodeMachine,RunResult};
use std::ops::{Range,Index,IndexMut};

#[derive(Debug)]
struct TwoVec<T> {
    negative: Vec<T>,
    positive: Vec<T>,
}

impl<T> TwoVec<T> {
    pub fn new(center_item: T) -> TwoVec<T> {
        TwoVec {
            negative: Vec::<T>::new(),
            positive: vec![ center_item ],
        }
    }

    pub fn expand_to_contain<F>(&mut self, index: i32, fill: F) where
        F: Fn() -> T {

        if index < 0 {
            while (self.negative.len() as i32) < -index {
                self.negative.push(fill());
            }
        }
        else {
            while (self.positive.len() as i32) < index + 1 {
                self.positive.push(fill());
            }
        }
    }

    pub fn index_range(&self) -> Range<i32> {
        Range {
            start: -(self.negative.len() as i32),
            end: self.positive.len() as i32
        }
    }
}

impl<T> Index<i32> for TwoVec<T> {
    type Output = T;

    fn index(&self, i: i32) -> &T {
        if i < 0 {
            &self.negative[-i as usize - 1]
        } else {
            &self.positive[ i as usize]
        }
    }
}

impl<T> IndexMut<i32> for TwoVec<T> {
    fn index_mut(&mut self, i: i32) -> &mut T {
        if i < 0 {
            &mut self.negative[-i as usize - 1]
        } else {
            &mut self.positive[ i as usize]
        }
    }
}

#[derive(Eq,PartialEq,Clone,Copy,Debug)]
enum PaintColor {
    Unpainted,
    Black,
    White
}

fn color_to_int(c: PaintColor) -> i64 {
    match c {
        PaintColor::Unpainted => 0,
        PaintColor::Black => 0,
        PaintColor::White => 1,
    }
}

fn int_to_color(i: i64) -> PaintColor {
    match i {
        0 => PaintColor::Black,
        1 => PaintColor::White,
        _ => panic!(),
    }
}

#[derive(Eq,PartialEq,Clone,Copy,Debug)]
enum TurnCommand {
    TurnLeft,
    TurnRight,
}

fn int_to_turn_cmd(i: i64) -> TurnCommand {
    match i {
        0 => TurnCommand::TurnLeft,
        1 => TurnCommand::TurnRight,
        _ => panic!(),
    }
}

#[derive(Debug)]
struct PaintBot {
    grid: TwoVec<TwoVec<PaintColor>>,
    position: (i32, i32),
    direction: (i32, i32),
}

impl PaintBot {
    pub fn new(start_color: PaintColor) -> PaintBot {
        PaintBot {
            grid: TwoVec::new(TwoVec::new(start_color)),
            position: (0, 0),
            direction: (0, 1),
        }
    }

    pub fn step(&mut self, paint: PaintColor, turn: TurnCommand) -> PaintColor {
        self.grid[self.position.0][self.position.1] = paint;

        match turn {
            TurnCommand::TurnLeft  => self.direction = (-self.direction.1,  self.direction.0),
            TurnCommand::TurnRight => self.direction = ( self.direction.1, -self.direction.0),
        };

        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;

        self.grid.expand_to_contain(self.position.0, || TwoVec::new(PaintColor::Unpainted));
        for i in self.grid.index_range() {
            self.grid[i].expand_to_contain(self.position.1, || PaintColor::Unpainted);
        }

        self.grid[self.position.0][self.position.1]
    }

    pub fn count_painted_tiles(&self) -> u32 {
        let mut result = 0u32;

        for x in self.grid.index_range() {
            for y in self.grid[x].index_range() {
                if self.grid[x][y] != PaintColor::Unpainted {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn render_image_to_string(&self) -> String {
        let mut result = String::new();

        for y in self.grid[0].index_range().rev() {
            for x in self.grid.index_range() {
                result.push_str(if self.grid[x][y] == PaintColor::White {
                    "X"
                } else {
                    " "
                });
            }
            result.push_str("\n");
        }

        result
    }
}

fn run_paint_bot(brain_tape: &[i64], start_color: PaintColor) -> PaintBot {
    let mut bot = PaintBot::new(start_color);
    let mut brain = IntCodeMachine::new(brain_tape);

    brain.run();
    brain.input_and_continue(color_to_int(start_color)).unwrap();

    while let Ok(_) = (|| {
        let color_command = brain.output_and_continue()?;
        let turn_command = brain.output_and_continue()?;
        let new_color = bot.step(int_to_color(color_command), int_to_turn_cmd(turn_command));
        brain.input_and_continue(color_to_int(new_color))
    })() {};

    bot
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day11.txt").unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let result0 = run_paint_bot(&tape, PaintColor::Unpainted).count_painted_tiles();
    let result1 = run_paint_bot(&tape, PaintColor::White).render_image_to_string();

    println!("{}\n\n{}", result0, result1);
}