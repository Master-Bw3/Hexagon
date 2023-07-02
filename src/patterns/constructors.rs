use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Stack, StackExt, State},
    },
    iota::{Iota, NumberIota},
    pattern_registry::{self, PatternRegistry},
};

type ActionType =
    dyn for<'a> Fn(&'a mut State, &PatternRegistry) -> Result<&'a mut State, Mishap>;

type GetterType = fn(&Stack, usize, usize) -> Result<NumberIota, Mishap>;

// pub fn spell(arg_count: &usize, getters: Vec<GetterType>) -> ActionType {
//     Box::new(
//         move |state: &mut State, pattern_registry: &PatternRegistry| {
//             for (index, getter) in getters.iter().enumerate() {
//                 getter(&state.stack, index, *arg_count);
//             }
//             state.stack.remove_args(arg_count);
//             Ok(state)
//         },
//     )
// }

pub fn push_const(iota: Iota) -> Box<ActionType> {
    Box::new(move |state: &mut State, _pattern_registry: &PatternRegistry| -> Result<&mut State, Mishap> {
        state.stack.push(iota.clone());
        Ok(state)
    })
}
// #[macro_export]

// macro_rules! push_const {
//     ( $x:expr  ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }
