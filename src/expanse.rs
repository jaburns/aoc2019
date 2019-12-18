use std::ops::{Index, IndexMut, Range};

#[derive(Debug)]
struct TwoVec<T> {
    negative: Vec<T>,
    positive: Vec<T>,
}

impl<T> TwoVec<T> {
    pub fn new() -> TwoVec<T> {
        TwoVec {
            negative: Vec::<T>::new(),
            positive: Vec::<T>::new(),
        }
    }

    pub fn expand_to_contain<F>(&mut self, index: i32, fill: F)
    where
        F: Fn() -> T,
    {
        if index < 0 {
            while (self.negative.len() as i32) < -index {
                self.negative.push(fill());
            }
        } else {
            while (self.positive.len() as i32) < index + 1 {
                self.positive.push(fill());
            }
        }
    }

    pub fn index_range(&self) -> Range<i32> {
        Range {
            start: -(self.negative.len() as i32),
            end: self.positive.len() as i32,
        }
    }
}

impl<T> Index<i32> for TwoVec<T> {
    type Output = T;

    fn index(&self, i: i32) -> &T {
        if i < 0 {
            &self.negative[-i as usize - 1]
        } else {
            &self.positive[i as usize]
        }
    }
}

impl<T> IndexMut<i32> for TwoVec<T> {
    fn index_mut(&mut self, i: i32) -> &mut T {
        if i < 0 {
            &mut self.negative[-i as usize - 1]
        } else {
            &mut self.positive[i as usize]
        }
    }
}

#[derive(Debug)]
pub struct Expanse<T> {
    grid: TwoVec<TwoVec<(bool, T)>>,
}

impl<T> Expanse<T> {
    pub fn new() -> Expanse<T> {
        Expanse {
            grid: TwoVec::new(),
        }
    }

    pub fn read(&self, x: i32, y: i32) -> Option<&T> {
        if self.grid.index_range().contains(&x) && self.grid[x].index_range().contains(&y) {
            let cell = &self.grid[x][y];
            if cell.0 {
                Some(&cell.1)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn write<F>(&mut self, x: i32, y: i32, item: T, filler: F) where
        F: Fn() -> T {

        self.grid.expand_to_contain(x, || TwoVec::new());
        for i in self.grid.index_range() {
            self.grid[i].expand_to_contain(y, || (false, filler()));
        }
        self.grid[x][y] = (true, item);
    }

    pub fn for_each<F>(&self, f: &mut F) where F: FnMut(i32,i32,Option<&T>) {
        for x in self.grid.index_range() {
            for y in self.grid[x].index_range() {
                f(x, y, self.read(x, y));
            }
        }
    }


    pub fn render_to_string<F>(&self, f: F) -> String where F: Fn(&T) -> &str {
        let mut result = String::new();

        for y in self.grid[0].index_range().rev() {
            for x in self.grid.index_range() {
                result.push_str(match self.read(x, y) {
                    Some(x) => f(x),
                    None => " ",
                });
            }
            result.push_str("\n");
        }

        result
    }
}