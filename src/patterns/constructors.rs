use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Stack, StackExt, State},
    },
    iota::{Iota},
    pattern_registry::PatternRegistry,
};

type ActionType = dyn for<'a> Fn(&'a mut State, &PatternRegistry) -> Result<&'a mut State, Mishap>;

type GetterType<T> = fn(&Stack, usize, usize) -> Result<T, Mishap>;

// pub fn spell<T:'static >(arg_count: usize, getters: Vec<GetterType<T>>) -> Box<ActionType> {
//     Box::new(
//         move |state: &mut State, _: &PatternRegistry| {
//             for (index, getter) in getters.iter().enumerate() {
//                 getter(&state.stack, index, arg_count)?;
//             }
//             state.stack.remove_args(&arg_count);
//             Ok(state)
//         },
//     )
// }

pub fn spell_1<T:'static >(getter: GetterType<T>) -> Box<ActionType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry| {
            getter(&state.stack, 0, 1)?;
            state.stack.remove_args(&1);
            Ok(state)
        },
    )
}

pub fn spell_2<T:'static, U:'static >(getter1: GetterType<T>, getter2: GetterType<U>) -> Box<ActionType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry| {
            getter1(&state.stack, 0, 2)?;
            getter2(&state.stack, 1, 2)?;

            state.stack.remove_args(&2);
            Ok(state)
        },
    )
}

pub fn spell_3<T:'static, U:'static, V:'static >(getter1: GetterType<T>, getter2: GetterType<U>, getter3: GetterType<V>) -> Box<ActionType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry| {
            getter1(&state.stack, 0, 3)?;
            getter2(&state.stack, 1, 3)?;
            getter3(&state.stack, 1, 3)?;


            state.stack.remove_args(&3);
            Ok(state)
        },
    )
}


pub fn push_const(iota: Iota) -> Box<ActionType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry| -> Result<&mut State, Mishap> {
            state.stack.push(iota.clone());
            Ok(state)
        },
    )
}
