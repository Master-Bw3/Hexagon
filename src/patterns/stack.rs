use nalgebra::ComplexField;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::{Iota, VectorIota},
    pattern_registry::PatternRegistry,
};

pub fn duplicate<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota(0, arg_count)?.clone();

    state.stack.push(iota);

    Ok(state)
}

pub fn two_dup<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_iota(1, arg_count)?.clone(),
    );

    state.stack.push(iotas.0);
    state.stack.push(iotas.1);

    Ok(state)
}

pub fn duplicate_n<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_number(1, arg_count)?.round() as usize,
    );
    state.stack.remove_args(&arg_count);

    state.stack.append(&mut vec![iotas.0; iotas.1]);

    Ok(state)
}

pub fn swap<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push(iotas.1);
    state.stack.push(iotas.0);

    Ok(state)
}

pub fn rotate<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_iota(1, arg_count)?.clone(),
        state.stack.get_iota(2, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push(iotas.1);
    state.stack.push(iotas.2);
    state.stack.push(iotas.0);

    Ok(state)
}

pub fn rotate_reverse<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_iota(1, arg_count)?.clone(),
        state.stack.get_iota(2, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push(iotas.2);
    state.stack.push(iotas.0);
    state.stack.push(iotas.1);

    Ok(state)
}

pub fn over<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push(iotas.0.clone());
    state.stack.push(iotas.1);
    state.stack.push(iotas.0);

    Ok(state)
}

pub fn tuck<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push(iotas.1.clone());
    state.stack.push(iotas.0);
    state.stack.push(iotas.1);

    Ok(state)
}

pub fn stack_len<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    state.stack.push(Iota::Number(state.stack.len() as f32));

    Ok(state)
}

pub fn fisherman<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    if state.stack.len() < 2 {
        return Err(Mishap::NotEnoughIotas(2));
    }

    let arg_count = 1;
    let iota = state.stack.get_integer(0, arg_count)?.clone();
    state.stack.remove_args(&arg_count);

    if state.stack.len() < iota as usize {
        return Err(Mishap::NotEnoughIotas(iota as usize));
    }

    if iota >= 0 {
        let iota = iota as usize;
        let operation_result = { state.stack[state.stack.len() - iota].clone() };

        state.stack.remove(state.stack.len() - iota);

        state.stack.push(operation_result);
    } else {
        let arg_count = 1;
        let iota2 = state.stack.get_iota(0, arg_count)?.clone();
        state.stack.remove_args(&arg_count);

        state.stack.insert(iota.abs() as usize, iota2)
    }

    Ok(state)
}

pub fn fisherman_copy<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    if state.stack.len() < 2 {
        return Err(Mishap::NotEnoughIotas(2));
    }

    let arg_count = 1;
    let iota = state.stack.get_integer(0, arg_count)?.clone() as usize;
    state.stack.remove_args(&arg_count);

    if state.stack.len() < iota {
        return Err(Mishap::NotEnoughIotas(iota));
    }

    let operation_result = { state.stack[state.stack.len() - 1 - iota].clone() };

    state.stack.push(operation_result);

    Ok(state)
}

// pub fn swizzle<'a>(
//     state: &'a mut State,
//     _pattern_registry: &PatternRegistry,
// ) -> Result<&'a mut State, Mishap> {
//     if state.stack.is_empty() {
//         return Err(Mishap::NotEnoughIotas(1));
//     }

//     let arg_count = 1;
//     let code = state
//         .stack
//         .get_positive_integer_under_inclusive(0, usize::MAX, arg_count)?
//         .clone();
//     state.stack.remove_args(&arg_count);

//     let mut strides: Vec<i32> = vec![];

//     for f in FactorialIter::new() {
//         if f <= code {
//             strides.push(f)
//         } else {
//             break;
//         }
//     }

//     if strides.len() > state.stack.len() {
//         return Err(Mishap::NotEnoughIotas(strides.len() + 1));
//     }

//     let mut edit_target =
//         &mut state.stack.clone()[(state.stack.len() - strides.len())..state.stack.len()];
//     let swap = edit_target.to_vec();
//     let mut radix = code;
//     let mut strides_reversed = strides.clone();
//     strides_reversed.reverse();
//     for divisor in strides_reversed {
//         let index = radix / divisor;
//         radix %= divisor;
//         edit_target[0] = swap[index as usize].clone();
//         let edit_target_len = edit_target.len();
//         edit_target = edit_target[1..edit_target_len].as_mut();
//         println!("edit: {:?}", edit_target);
//     }

//     println!("stack: {:?}, \n edit: {:?}", state.stack, edit_target);

//     Ok(state)
// }

struct FactorialIter {
    acc: i32,
    n: i32,
}

impl FactorialIter {
    fn new() -> FactorialIter {
        FactorialIter { acc: 1, n: 1 }
    }
}

impl Iterator for FactorialIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.acc;
        self.acc *= self.n;
        self.n += 1;
        Some(out)
    }
}

mod tests {

    use crate::pattern_registry::PatternRegistryExt;

    use super::*;

    #[test]
    fn rotate_test() {
        let mut state = State::default();
        state.stack = vec![Iota::Number(0.0), Iota::Number(1.0), Iota::Number(2.0)];

        let expected = vec![Iota::Number(1.0), Iota::Number(2.0), Iota::Number(0.0)];

        let result = rotate(&mut state, &PatternRegistry::construct()).unwrap();
        assert_eq!(result.stack, expected)
    }

    #[test]
    fn rotate_reverse_test() {
        let mut state = State::default();
        state.stack = vec![Iota::Number(0.0), Iota::Number(1.0), Iota::Number(2.0)];

        let expected = vec![Iota::Number(2.0), Iota::Number(0.0), Iota::Number(1.0)];

        let result = rotate_reverse(&mut state, &PatternRegistry::construct()).unwrap();
        assert_eq!(result.stack, expected)
    }

    #[test]
    fn over_test() {
        let mut state = State::default();
        state.stack = vec![Iota::Number(0.0), Iota::Number(1.0)];

        let expected = vec![Iota::Number(0.0), Iota::Number(1.0), Iota::Number(0.0)];

        let result = over(&mut state, &PatternRegistry::construct()).unwrap();
        assert_eq!(result.stack, expected)
    }

    #[test]
    fn tuck_test() {
        let mut state = State::default();
        state.stack = vec![Iota::Number(0.0), Iota::Number(1.0)];

        let expected = vec![Iota::Number(1.0), Iota::Number(0.0), Iota::Number(1.0)];

        let result = tuck(&mut state, &PatternRegistry::construct()).unwrap();
        assert_eq!(result.stack, expected)
    }

    #[test]
    fn fisherman_test() {
        let mut state = State::default();
        state.stack = vec![
            Iota::Number(0.0),
            Iota::Number(1.0),
            Iota::Number(2.0),
            Iota::Number(2.0),
        ];

        let expected = vec![Iota::Number(0.0), Iota::Number(2.0), Iota::Number(1.0)];

        let result = fisherman(&mut state, &PatternRegistry::construct()).unwrap();
        assert_eq!(result.stack, expected)
    }

    fn fisherman_neg_test() {
        todo!("negative fisherman isn't a thing yet")
    }

    #[test]
    fn fisherman_copy_test() {
        let mut state = State::default();
        state.stack = vec![
            Iota::Number(0.0),
            Iota::Number(1.0),
            Iota::Number(2.0),
            Iota::Number(2.0),
        ];

        let expected = vec![
            Iota::Number(0.0),
            Iota::Number(1.0),
            Iota::Number(2.0),
            Iota::Number(0.0),
        ];

        let result = fisherman_copy(&mut state, &PatternRegistry::construct()).unwrap();
        assert_eq!(result.stack, expected)
    }
}

