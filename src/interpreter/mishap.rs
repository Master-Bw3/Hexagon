use std::rc::Rc;

use im::Vector;

use crate::{
    interpreter::state::Stack,
    iota::{
        hex_casting::vector::VectorIota,
        hex_casting::{garbage::GarbageIota, pattern::PatternIota},
        Iota,
    }, parser::Location,
};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Mishap {
    NotEnoughIotas(usize, usize),
    IncorrectIota(usize, String, Rc<dyn Iota>),
    MathematicalError(),
    HastyRetrospection,
    InvalidPattern,
    ExpectedPattern(Rc<dyn Iota>),
    ExpectedValue(String, String),
    InvalidValue(String, String),
    OpCannotBeConsidered,
    OpNotEnoughArgs(i32),
    OpExpectedVar(Rc<dyn Iota>),
    OpExpectedIota,
    VariableNotAssigned(String),
    NoIotaAtIndex(usize),
    NoAkashicRecord(Rc<VectorIota>),
    HoldingIncorrectItem,
    MatrixWrongSize(Rc<dyn Iota>, MatrixSize, MatrixSize),
}

impl Mishap {
    pub fn apply_to_stack(self, stack: Stack) -> Stack {
        match self {
            Mishap::NotEnoughIotas(_, num) => {
                let mut new_stack = stack;
                let garbage = Rc::new(GarbageIota);
                let garbages: Vec<Rc<dyn Iota>> = vec![garbage; num];
                new_stack.append(Vector::from(garbages));
                new_stack
            }
            Mishap::IncorrectIota(index, _, _) => {
                let mut new_stack = stack;
                new_stack[index] = Rc::new(GarbageIota);
                new_stack
            }
            Mishap::MathematicalError() => todo!(),
            Mishap::HastyRetrospection => {
                let retro_sig: &str = "eee";
                let mut new_stack = stack;
                new_stack.push_back(Rc::new(PatternIota::from_sig(retro_sig, None, Location::Unknown)));
                new_stack
            }
            Mishap::InvalidPattern => {
                let mut new_stack = stack;
                new_stack.push_back(Rc::new(GarbageIota));
                new_stack
            }
            Mishap::ExpectedPattern(_) => todo!(),

            Mishap::OpCannotBeConsidered => todo!(),
            Mishap::OpNotEnoughArgs(_) => todo!(),
            Mishap::OpExpectedVar(_) => todo!(),
            Mishap::VariableNotAssigned(_) => todo!(),
            Mishap::OpExpectedIota => todo!(),
            Mishap::NoIotaAtIndex(_) => todo!(),
            Mishap::NoAkashicRecord(_) => todo!(),
            Mishap::HoldingIncorrectItem => todo!(),
            Mishap::ExpectedValue(_, _) => todo!(),
            Mishap::InvalidValue(_, _) => todo!(),
            Mishap::MatrixWrongSize(_, _, _) => todo!(),
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            Mishap::NotEnoughIotas(arg_count, stack_height) => format!(
                "Expected {arg_count} arguments but the stack was only {stack_height} tall"
            ),
            Mishap::IncorrectIota(index, expected, recieved) => format!(
                "expected {} at index {index} of the stack, but got {}",
                expected,
                recieved.display()
            ),
            Mishap::MathematicalError() => todo!(),
            Mishap::HastyRetrospection => "Expected preceding Introspection".to_string(),
            Mishap::InvalidPattern => "This pattern isn't associated with any action".to_string(),
            Mishap::ExpectedPattern(iota) => format!("Expected Pattern but got {}", iota.display()),
            Mishap::OpCannotBeConsidered => "Ops cannot be considered".to_string(),
            Mishap::OpNotEnoughArgs(arg_count) => format!("Expected {arg_count} arguments"),
            Mishap::OpExpectedVar(iota) => format!(
                "Expected argument to be a variable but got iota {}",
                iota.display()
            ),
            Mishap::OpExpectedIota => "Expected argument to be an iota".to_string(),
            Mishap::VariableNotAssigned(_) => "Variable never assigned".to_string(),
            Mishap::NoIotaAtIndex(_) => "No iota found at pointed location".to_string(),
            Mishap::NoAkashicRecord(location) => format!("No akashic record found at {location}"),
            Mishap::HoldingIncorrectItem => "Entity is not holding the right item".to_string(),
            Mishap::ExpectedValue(_, expected) => {
                format!("Expected {expected} value to be supplied but got Nothing")
            }
            Mishap::InvalidValue(expected, recieved) => {
                format!("Expected {expected} value to be supplied but got {recieved}")
            }
            Mishap::MatrixWrongSize(iota, row_count, col_count) => format!(
                "Expected {row_count} by {col_count} matrix but found {}",
                iota.display()
            ),
        }
    }

    pub fn error_hint(&self) -> Option<String> {
        match self {
            Mishap::NotEnoughIotas(_arg_count, _stack_height) => None,
            Mishap::IncorrectIota(_index, _expected, _recieved) => None,
            Mishap::MathematicalError() => None,
            Mishap::HastyRetrospection => None,
            Mishap::InvalidPattern => None,
            Mishap::ExpectedPattern(_iota) => None,
            Mishap::OpCannotBeConsidered => None,
            Mishap::OpNotEnoughArgs(_arg_count) => {
                Some("Provide arguments inside the parentheses: Op(arg)".to_string())
            }
            Mishap::OpExpectedVar(_iota) => {
                Some("Use a variable as the argument: Op($var)".to_string())
            }
            Mishap::OpExpectedIota => {
                Some("Use an Iota as the argument: Op(1), Op([1, 1, 1]), ect.".to_string())
            }
            Mishap::VariableNotAssigned(varname) => Some(format!(
                "Assign the variable using Store({varname}) or Copy({varname})"
            )),
            Mishap::NoIotaAtIndex(_) => Some(
                "This is typically caused by the Ravenmind being overwritten via Huginn's Gambit"
                    .to_string(),
            ),
            Mishap::NoAkashicRecord(_location) => {
                Some("Define an akashic record in a 'config.toml' file".to_string())
            }
            Mishap::HoldingIncorrectItem => {
                Some("Define held items in a 'config.toml' file".to_string())
            }
            //TODO: make expectedValue show iota instead of type of iota in example
            Mishap::ExpectedValue(action_name, expected) => Some(format!(
                "Set a value for this action. Example: {action_name}: {expected}"
            )),
            Mishap::InvalidValue(_expected, _recieved) => None,
            Mishap::MatrixWrongSize(_, _, _) => None,
        }
    }
}
