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
                "expected {:?} at index {index} of the stack, but got {:?}",
                expected, recieved
            ),
            Mishap::MathematicalError() => todo!(),
            Mishap::HastyRetrospection => "Expected preceding Introspection".to_string(),
            Mishap::InvalidPattern => "That pattern isn't associated with any action".to_string(),
            Mishap::ExpectedPattern(iota) => format!("Expected Pattern but got {:?}", iota),
            Mishap::OpCannotBeConsidered => todo!(),
            Mishap::OpNotEnoughArgs(_) => todo!(),
            Mishap::OpExpectedVar(_) => todo!(),
            Mishap::OpExpectedIota => todo!(),
            Mishap::VariableNotAssigned => todo!(),
            Mishap::NoIotaAtIndex(_) => todo!(),
            Mishap::NoAkashicRecord(_) => todo!(),
            Mishap::HoldingIncorrectItem => todo!(),
            Mishap::ExpectedValue(expected) => todo!(),
            Mishap::InvalidValue(expected, recieved) => todo!(),
        }
    }
}