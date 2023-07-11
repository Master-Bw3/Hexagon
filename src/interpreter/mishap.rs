use crate::iota::GarbageIota::Garbage;
use crate::iota::{PatternIota, VectorIota};
use crate::{interpreter::state::Stack, iota::Iota};

#[derive(Debug)]
pub enum Mishap {
    NotEnoughIotas(usize),
    IncorrectIota(usize),
    MathematicalError(),
    HastyRetrospection,
    InvalidPattern,
    ExpectedPattern(Iota),
    ExpectedValue,
    InvalidValue,
    OpCannotBeConsidered,
    OpNotEnoughArgs(i32),
    OpExpectedVar(Iota),
    OpExpectedIota,
    VariableNotAssigned,
    NoIotaAtIndex(usize),
    NoAkashicRecord(VectorIota),
    HoldingIncorrectItem
}

impl Mishap {
    pub fn apply_to_stack(self, stack: Stack) -> Stack {
        match self {
            Mishap::NotEnoughIotas(num) => {
                let mut new_stack = stack;
                new_stack.append(&mut vec![Iota::Garbage(Garbage); num]);
                new_stack
            }
            Mishap::IncorrectIota(index) => {
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
            Mishap::ExpectedValue => todo!(),
            Mishap::InvalidValue => todo!(),
            Mishap::OpCannotBeConsidered => todo!(),
            Mishap::OpNotEnoughArgs(_) => todo!(),
            Mishap::OpExpectedVar(_) => todo!(),
            Mishap::VariableNotAssigned => todo!(),
            Mishap::OpExpectedIota => todo!(),
            Mishap::NoIotaAtIndex(_) => todo!(),
            Mishap::NoAkashicRecord(_) => todo!(),
            Mishap::HoldingIncorrectItem => todo!(),
        }
    }
}
