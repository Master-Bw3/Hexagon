use std::rc::Rc;

use im::Vector;

use crate::{
    interpreter::state::Stack,
    iota::{
        hex_casting::vector::VectorIota,
        hex_casting::{garbage::GarbageIota, pattern::PatternIota},
        Iota,
    },
    parser::Location,
};

#[derive(Debug, Clone)]
pub enum MatrixSize {
    N,
    Const(usize),
    Max(usize),
}

impl std::fmt::Display for MatrixSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixSize::N => write!(f, "n"),
            MatrixSize::Const(len) => write!(f, "{len}"),
            MatrixSize::Max(len) => write!(f, "(max {len})"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Mishap {
    NotEnoughIotas {
        arg_count: usize,
        stack_height: usize,
    },
    IncorrectIota {
        index: usize,
        expected: String,
        received: Rc<dyn Iota>,
    },
    MathematicalError,
    HastyRetrospection,
    InvalidPattern,
    ExpectedPattern {
        iota: Rc<dyn Iota>,
    },
    ExpectedValue {
        caused_by: String,
        expected: String,
    },
    InvalidValue {
        expected: String,
        received: String,
    },
    OpCannotBeConsidered,
    OpNotEnoughArgs {
        arg_count: i32,
    },
    OpExpectedVar {
        received: Rc<dyn Iota>,
    },
    OpExpectedIota,
    VariableNotAssigned {
        variable_name: String,
    },
    NoIotaAtIndex {
        index: usize,
    },
    NoAkashicRecord {
        location: Rc<VectorIota>,
    },
    HoldingIncorrectItem,
    MatrixWrongSize {
        iota: Rc<dyn Iota>,
        row_count: MatrixSize,
        col_count: MatrixSize,
    },
}

impl Mishap {
    pub fn apply_to_stack(&self, stack: &Stack) -> Stack {
        match self {
            Mishap::NotEnoughIotas {
                arg_count: _,
                stack_height,
            } => {
                let mut new_stack = stack.clone();
                let garbage = Rc::new(GarbageIota);
                let garbages: Vec<Rc<dyn Iota>> = vec![garbage; *stack_height];
                new_stack.append(Vector::from(garbages));
                new_stack
            }
            Mishap::IncorrectIota {
                index,
                expected: _,
                received: _,
            } => {
                let mut new_stack = stack.clone();
                new_stack[*index] = Rc::new(GarbageIota);
                new_stack
            }
            Mishap::MathematicalError => todo!(),
            Mishap::HastyRetrospection => {
                let retro_sig: &str = "eee";
                let mut new_stack = stack.clone();
                new_stack.push_back(Rc::new(PatternIota::from_sig(
                    retro_sig,
                    None,
                    Location::Unknown,
                )));
                new_stack
            }
            Mishap::InvalidPattern => {
                let mut new_stack = stack.clone();
                new_stack.push_back(Rc::new(GarbageIota));
                new_stack
            }
            Mishap::ExpectedPattern { iota } => todo!(),

            Mishap::OpCannotBeConsidered => todo!(),
            Mishap::OpNotEnoughArgs { arg_count } => todo!(),
            Mishap::OpExpectedVar { received: expected } => todo!(),
            Mishap::VariableNotAssigned { variable_name } => todo!(),
            Mishap::OpExpectedIota => todo!(),
            Mishap::NoIotaAtIndex { index } => todo!(),
            Mishap::NoAkashicRecord { location } => todo!(),
            Mishap::HoldingIncorrectItem => todo!(),
            Mishap::ExpectedValue {
                caused_by,
                expected,
            } => stack.clone(),
            Mishap::InvalidValue { expected, received: recieved } => todo!(),
            Mishap::MatrixWrongSize {
                iota,
                row_count,
                col_count,
            } => todo!(),
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            Mishap::NotEnoughIotas {
                arg_count,
                stack_height,
            } => {
                format!("Expected {arg_count} arguments but the stack was only {stack_height} tall")
            }
            Mishap::IncorrectIota {
                index,
                expected,
                received: recieved,
            } => format!(
                "expected {} at index {index} of the stack, but got {}",
                expected,
                recieved.display()
            ),
            Mishap::MathematicalError => todo!(),
            Mishap::HastyRetrospection => "Expected preceding Introspection".to_string(),
            Mishap::InvalidPattern => "This pattern isn't associated with any action".to_string(),
            Mishap::ExpectedPattern { iota } => {
                format!("Expected Pattern but got {}", iota.display())
            }
            Mishap::OpCannotBeConsidered => "Ops cannot be considered".to_string(),
            Mishap::OpNotEnoughArgs { arg_count } => format!("Expected {arg_count} arguments"),
            Mishap::OpExpectedVar { received: expected } => format!(
                "Expected argument to be a variable but got iota {}",
                expected.display()
            ),
            Mishap::OpExpectedIota => "Expected argument to be an iota".to_string(),
            Mishap::VariableNotAssigned { variable_name } => "Variable never assigned".to_string(),
            Mishap::NoIotaAtIndex { index } => "No iota found at pointed location".to_string(),
            Mishap::NoAkashicRecord { location } => {
                format!("No akashic record found at {location}")
            }
            Mishap::HoldingIncorrectItem => "Entity is not holding the right item".to_string(),
            Mishap::ExpectedValue {
                caused_by,
                expected,
            } => {
                format!("Expected {expected} value to be supplied but got Nothing")
            }
            Mishap::InvalidValue { expected, received: recieved } => {
                format!("Expected {expected} value to be supplied but got {recieved}")
            }
            Mishap::MatrixWrongSize {
                iota,
                row_count,
                col_count,
            } => format!(
                "Expected {row_count} by {col_count} matrix but found {}",
                iota.display()
            ),
        }
    }

    pub fn error_hint(&self) -> Option<String> {
        match self {
            Mishap::NotEnoughIotas {
                arg_count,
                stack_height,
            } => None,
            Mishap::IncorrectIota {
                index,
                expected,
                received: recieved,
            } => None,
            Mishap::MathematicalError => None,
            Mishap::HastyRetrospection => None,
            Mishap::InvalidPattern => None,
            Mishap::ExpectedPattern { iota } => None,
            Mishap::OpCannotBeConsidered => None,
            Mishap::OpNotEnoughArgs { arg_count } => {
                Some("Provide arguments inside the parentheses: Op(arg)".to_string())
            }
            Mishap::OpExpectedVar { received: expected } => {
                Some("Use a variable as the argument: Op($var)".to_string())
            }
            Mishap::OpExpectedIota => {
                Some("Use an Iota as the argument: Op(1), Op([1, 1, 1]), ect.".to_string())
            }
            Mishap::VariableNotAssigned { variable_name } => Some(format!(
                "Assign the variable using Store({variable_name}) or Copy({variable_name})"
            )),
            Mishap::NoIotaAtIndex { index } => Some(
                "This is typically caused by the Ravenmind being overwritten via Huginn's Gambit"
                    .to_string(),
            ),
            Mishap::NoAkashicRecord { location } => {
                Some("Define an akashic record in a 'config.toml' file".to_string())
            }
            Mishap::HoldingIncorrectItem => {
                Some("Define held items in a 'config.toml' file".to_string())
            }
            //TODO: make expectedValue show iota instead of type of iota in example
            Mishap::ExpectedValue {
                caused_by,
                expected,
            } => Some(format!(
                "Set a value for this action. Example: {}: {}",
                caused_by, expected
            )),
            Mishap::InvalidValue { expected, received: recieved } => None,
            Mishap::MatrixWrongSize {
                iota,
                row_count,
                col_count,
            } => None,
        }
    }
}
