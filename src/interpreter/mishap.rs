use crate::iota::GarbageIota::Garbage;
use crate::{interpreter::state::Stack, iota::Iota};

#[derive(Debug)]
pub enum Mishap {
    NotEnoughIotas(usize),
    IncorrectIota(usize),
    MathematicalError(),
}

impl Mishap {
    pub fn apply_to_stack(self, stack: Stack) -> Stack {
        match self {
            Mishap::NotEnoughIotas(num) => {
                let mut new_stack = stack.clone();
                new_stack.append(&mut vec![Iota::Garbage(Garbage); num]);
                new_stack
            }
            Mishap::IncorrectIota(index) => {
                let mut new_stack = stack.clone();
                new_stack[index] = Iota::Garbage(Garbage);
                new_stack
            }
            Mishap::MathematicalError() => todo!(),
        }
    }
}
