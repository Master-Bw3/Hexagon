use im::Vector;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::hex_casting::number::{NumberIota, NumberIotaExt},
    pattern_registry::PatternRegistry,
};

/** iterator yielding 1, 1, 2, 6, 24, ... */
struct Factorial {
    acc: usize,
    n: usize,
}

impl Factorial {
    fn new() -> Self {
        Factorial { acc: 1, n: 1 }
    }
}

impl Iterator for Factorial {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let val = self.acc;
        self.acc *= self.n;
        self.n += 1;
        Some(val)
    }
}

/** Fn to take a stack and a value and perform a shuffling of the last few elems */
fn fixed_factorial<T: Clone>(mut value: usize, stack: &mut [T]) -> Result<(), Mishap> {
    let mut strides: Vec<usize> = Factorial::new().take_while(|&x| x <= value).collect();

    // want only the last few elems of the stack
    if stack.len() < strides.len() {
        Err(Mishap::NotEnoughIotas{arg_count: strides.len(), stack_height: stack.len()})?
    }
    let stride_offset = stack.len() - strides.len();
    let mut edit_target = &mut stack[stride_offset..];
    let mut swap = edit_target.to_vec();

    while let Some(divisor) = strides.pop() {
        let index = value / divisor;
        value %= divisor;
        edit_target[0] = swap.remove(index);
        edit_target = &mut edit_target[1..];
    }

    Ok(())
}

pub fn swizzle<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    if state.stack.is_empty() {
        return Err(Mishap::NotEnoughIotas { arg_count: 1, stack_height: state.stack.len() });
    }

    let arg_count = 1;
    let code = state.stack.get_iota::<NumberIota>(0, arg_count)?.int(0)? as usize;
    state.stack.remove_args(&arg_count);

    let mut new_stack = state.stack.iter().cloned().collect::<Vec<_>>();
    fixed_factorial(code, &mut new_stack)?;
    state.stack = Vector::from(new_stack);

    Ok(state)
}

// #[cfg(test)]
// mod tests {

//     use crate::{pattern_registry::PatternRegistryExt, iota::Iota};

//     use super::*;

//     #[test]
//     fn swizzle_test() {
//         let mut state = State::default();
//         state.stack = vec![
//             Iota::Number(2.0),
//             Iota::Number(1.0),
//             Iota::Number(0.0),
//             //code
//             Iota::Number(5.0),
//         ];

//         let expected = vec![Iota::Number(0.0), Iota::Number(1.0), Iota::Number(2.0)];

//         let result = swizzle(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }
// }
