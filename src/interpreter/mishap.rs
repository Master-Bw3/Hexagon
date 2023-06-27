use crate::interpreter::stack::{Stack, StackExt};

use super::stack::State;


pub enum Mishap {
    NotEnoughIotas(i32),
    IncorrectIota(usize),
    MathematicalError(),
}

impl Mishap {
    fn apply_mishap(state: State) -> State {
        let t = Stack::new();
        let e: Result<bool, Mishap> = t.get_bool_iota(1);
        todo!()

    }
}