use crate::solution::{Implementations, UnableToExecuteActError};

pub struct Day2;

impl Day2 {
    pub fn new() -> Self {
        Day2
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Implementations for Day2 {
    fn act_1(&self) -> Result<(), crate::solution::UnableToExecuteActError> {
        Err(UnableToExecuteActError::NotImplemented)
    }
    fn act_2(&self) -> Result<(), UnableToExecuteActError> {
        Err(UnableToExecuteActError::NotImplemented)
    }
}
