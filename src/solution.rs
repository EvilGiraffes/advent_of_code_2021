use std::fmt::{self, Debug, Display, Formatter};

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

pub trait ExecutionContext: Display + Debug {}
impl<T> ExecutionContext for T where T: Display + Debug {}


#[derive(Debug, Clone)]
pub enum UnableToExecuteActError {
    NotImplemented,
    FailedToExecute(Option<&'static dyn ExecutionContext>),
}

impl Display for UnableToExecuteActError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotImplemented => write!(f, "not implemented"),
            Self::FailedToExecute(reason) => {
                match reason {
                    Some(inner) => write_seperated(f, " ", &inner),
                    None => write_seperated(f, "", ""),
                }
            }
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

fn write_seperated<T>(f: &mut Formatter<'_>,seperator: &str, value: &T) -> fmt::Result
where T : Display + ?Sized{
    write!(f, "failed to execute act{}{}", seperator, value)
}
