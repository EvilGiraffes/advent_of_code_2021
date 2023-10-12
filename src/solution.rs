use std::fmt::Display;

pub trait Solution {
    fn execute_act(&self, act: Act) -> Result<(), UnableToExecuteActError>;
}

pub trait Implementations {
    fn act_1(&self) -> Result<(), UnableToExecuteActError>;
    fn act_2(&self) -> Result<(), UnableToExecuteActError>;
}

impl<T> Solution for T
where
    T: Implementations,
{
    fn execute_act(&self, act: Act) -> Result<(), UnableToExecuteActError> {
        match act {
            Act::One => self.act_1(),
            Act::Two => self.act_2(),
        }
    }
}

fn extract_reason(optional_reason: &Option<String>) -> String {
    match optional_reason {
        Some(reason) => reason.clone(),
        None => "".to_owned(),
    }
}

#[derive(Debug, Clone)]
pub enum UnableToExecuteActError {
    NotImplemented,
    FailedToExecute(Option<String>),
}


impl Display for UnableToExecuteActError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented => write!(f, "not implemented"),
            Self::FailedToExecute(reason) => write!(f, "failed to execute act{}", extract_reason(&reason)),
        }
    }
}

pub enum Act {
    One,
    Two,
}

impl Act {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::One),
            2 => Some(Self::Two),
            _ => None,
        }
    }
}
