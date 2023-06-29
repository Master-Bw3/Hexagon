use crate::iota::GarbageIota::Garbage;
use crate::iota::{PatternIota, PatternIotaExt};
use crate::{interpreter::state::Stack, iota::Iota};

#[derive(Debug)]
pub enum Mishap {
    NotEnoughIotas(usize),
    IncorrectIota(usize),
    MathematicalError(),
    HastyRetrospection,
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
            Mishap::HastyRetrospection => {
                let retro_sig: &str = "eee";
                let mut new_stack = stack.clone();
                new_stack.push(Iota::Pattern(PatternIota::from_sig(&retro_sig, None)));
                new_stack
            }
        }
    }
}
