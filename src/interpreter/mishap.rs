use crate::iota::GarbageIota::Garbage;
use crate::iota::{PatternIota, VectorIota};
use crate::{interpreter::state::Stack, iota::Iota};

#[derive(Debug)]
pub enum Mishap {
    NotEnoughIotas(usize, usize),
    IncorrectIota(usize, String, Iota),
    MathematicalError(),
    HastyRetrospection,
    InvalidPattern,
    ExpectedPattern(Iota),
    ExpectedValue(String),
    InvalidValue(String, String),
    OpCannotBeConsidered,
    OpNotEnoughArgs(i32),
    OpExpectedVar(Iota),
    OpExpectedIota,
    VariableNotAssigned,
    NoIotaAtIndex(usize),
    NoAkashicRecord(VectorIota),
    HoldingIncorrectItem,
}

impl Mishap {
    pub fn apply_to_stack(self, stack: Stack) -> Stack {
        match self {
            Mishap::NotEnoughIotas(_, num) => {
                let mut new_stack = stack;
                new_stack.append(&mut vec![Iota::Garbage(Garbage); num]);
                new_stack
            }
            Mishap::IncorrectIota(index, _, _) => {
                let mut new_stack = stack;
                new_stack[index] = Iota::Garbage(Garbage);
                new_stack
            }
            Mishap::MathematicalError() => todo!(),
            Mishap::HastyRetrospection => {
                let retro_sig: &str = "eee";
                let mut new_stack = stack;
                new_stack.push(Iota::Pattern(PatternIota::from_sig(retro_sig, None)));
                new_stack
            }
            Mishap::InvalidPattern => {
                let mut new_stack = stack;
                new_stack.push(Iota::Garbage(Garbage));
                new_stack
            }
            Mishap::ExpectedPattern(_) => todo!(),

            Mishap::OpCannotBeConsidered => todo!(),
            Mishap::OpNotEnoughArgs(_) => todo!(),
            Mishap::OpExpectedVar(_) => todo!(),
            Mishap::VariableNotAssigned => todo!(),
            Mishap::OpExpectedIota => todo!(),
            Mishap::NoIotaAtIndex(_) => todo!(),
            Mishap::NoAkashicRecord(_) => todo!(),
            Mishap::HoldingIncorrectItem => todo!(),
            Mishap::ExpectedValue(_) => todo!(),
            Mishap::InvalidValue(_, _) => todo!(),
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            Mishap::NotEnoughIotas(arg_count, stack_height) => format!("Expected {arg_count} or more arguments but the stack was only {stack_height} tall"),
            Mishap::IncorrectIota(index, expected, recieved) => format!(
                "expected {} at index {index} of the stack, but got {}",
                expected, recieved.display()
            ),
            Mishap::MathematicalError() => todo!(),
            Mishap::HastyRetrospection => "Expected preceding Introspection".to_string(),
            Mishap::InvalidPattern => "That pattern isn't associated with any action".to_string(),
            Mishap::ExpectedPattern(iota) => format!("Expected Pattern but got {}", iota.display()),
            Mishap::OpCannotBeConsidered => "Ops cannot be considered".to_string(),
            Mishap::OpNotEnoughArgs(arg_count) => format!("Expected {arg_count} arguments"),
            Mishap::OpExpectedVar(iota) => format!("Expected argument to be a variable but got iota {}", iota.display()),
            Mishap::OpExpectedIota => "Expected argument to be an iota".to_string(),
            Mishap::VariableNotAssigned => "Variable never assigned".to_string(),
            Mishap::NoIotaAtIndex(_) => "No iota found at pointed location".to_string(),
            Mishap::NoAkashicRecord(location) => format!("No akashic record found at {location}"),
            Mishap::HoldingIncorrectItem => "Entity is not holding the right item".to_string(),
            Mishap::ExpectedValue(expected) => format!("Expected {expected} but got Nothing"),
            Mishap::InvalidValue(expected, recieved) =>  format!("Expected {expected} but got {recieved}"),
        }
    }
}