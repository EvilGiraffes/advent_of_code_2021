use crate::solution::{Implementations, UnableToExecuteActError};
use std::{collections::VecDeque, fs, path::Path};

pub struct Day1 {
    input: Vec<u32>,
}

impl Day1 {
    pub fn new() -> Self {
        let input_path = Path::new("./src/day_1/input.txt");
        Day1 {
            input: fs::read_to_string(input_path)
                .expect("Cant read file.")
                .lines()
                .map(String::from)
                .map(|str| str.parse::<u32>())
                .map(|result| result.expect("Can't parse the the number."))
                .collect(),
        }
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Implementations for Day1 {
    // Result : 1711
    fn act_1(&self) -> Result<(), UnableToExecuteActError> {
        let result = self
            .input
            .iter()
            .fold(
                PreviousContext::default(),
                |acc: PreviousContext, current: &u32| -> _ {
                    if acc.previous >= *current {
                        return acc.with_new_current(*current);
                    }
                    acc.with_incremented_count(*current)
                },
            )
            .count;
        println!("{}", result);
        Ok(())
    }
    // Result : 1743
    fn act_2(&self) -> Result<(), UnableToExecuteActError> {
        let size = 3;
        let result = self
            .input
            .iter()
            .scan(Window::new(size), |acc, x| -> _ {
                acc.push(*x);
                Some((acc.len().clone(), acc.sum().clone()))
            })
            .filter_map(|x| -> _ {
                let len = x.0;
                let sum = x.1;
                if len < size {
                    return None;
                }
                Some(sum)
            })
            .fold(PreviousContext::default(), |acc: PreviousContext, current: u32| -> _ {
                if acc.previous >= current {
                    return acc.with_new_current(current);
                }
                acc.with_incremented_count(current)
            })
            .count;
        println!("{}", result);
        Ok(())
    }
}

struct PreviousContext {
    pub previous: u32,
    pub count: u32,
}

impl PreviousContext {
    pub fn with_new_current(&self, current: u32) -> Self {
        PreviousContext {
            previous: current,
            count: self.count,
        }
    }
    pub fn with_incremented_count(&self, current: u32) -> Self {
        PreviousContext {
            previous: current,
            count: self.count + 1,
        }
    }
}

impl Default for PreviousContext{
    fn default() -> Self {
        PreviousContext {
            previous: u32::MAX,
            count: 0,
        }
    }
}

#[derive(Debug)]
enum WindowError {
    UninitalizedWindow(Option<String>),
}

struct Window {
    contained: VecDeque<u32>,
    max: usize,
    sum: u32,
}

impl Window {
    pub fn new(max: usize) -> Self {
        Window {
            contained: VecDeque::with_capacity(max),
            max,
            sum: 0,
        }
    }
    pub fn sum(&self) -> &u32 {
        &self.sum
    }
    pub fn len(&self) -> usize {
        self.contained.len()
    }
    pub fn push(&mut self, item: u32) {
        if self.len() >= self.max {
            self.push_initialized(item).unwrap();
            return;
        }
        self.push_naive(item)
    }
    fn push_naive(&mut self, item: u32) {
        self.sum += item;
        self.contained.push_front(item)
    }
    fn push_initialized(&mut self, item: u32) -> Result<(), WindowError> {
        let previous = self.contained.pop_back().ok_or_else(|| {
            WindowError::UninitalizedWindow(Some(
                "Cannot push into window when it is uninitialized".to_owned(),
            ))
        })?;
        self.sum -= previous;
        self.push_naive(item);
        Ok(())
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new(3)
    }
}
