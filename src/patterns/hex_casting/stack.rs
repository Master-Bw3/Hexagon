use std::rc::Rc;

use im::{Vector, vector};

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{Iota, hex_casting::number::{NumberIota, NumberIotaExt}},
    parser::ActionValue,
    pattern_registry::PatternRegistry,
};

pub fn duplicate<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_any_iota(0, arg_count)?.clone();

    state.stack.push_back(iota);

    Ok(state)
}

pub fn two_dup<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );

    state.stack.push_back(iotas.0);
    state.stack.push_back(iotas.1);

    Ok(state)
}

pub fn duplicate_n<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_iota::<NumberIota>(1, arg_count)?.round() as usize,
    );
    state.stack.remove_args(&arg_count);

    state.stack.append(Vector::from(vec![iotas.0; iotas.1]));

    Ok(state)
}

pub fn swap<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push_back(iotas.1);
    state.stack.push_back(iotas.0);

    Ok(state)
}

pub fn rotate<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
        state.stack.get_any_iota(2, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push_back(iotas.1);
    state.stack.push_back(iotas.2);
    state.stack.push_back(iotas.0);

    Ok(state)
}

pub fn rotate_reverse<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
        state.stack.get_any_iota(2, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push_back(iotas.2);
    state.stack.push_back(iotas.0);
    state.stack.push_back(iotas.1);

    Ok(state)
}

pub fn over<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push_back(iotas.0.clone());
    state.stack.push_back(iotas.1);
    state.stack.push_back(iotas.0);

    Ok(state)
}

pub fn tuck<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push_back(iotas.1.clone());
    state.stack.push_back(iotas.0);
    state.stack.push_back(iotas.1);

    Ok(state)
}

pub fn stack_len<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    state.stack.push_back(Rc::new(state.stack.len() as f32));

    Ok(state)
}

pub fn fisherman<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    if state.stack.len() < 2 {
        return Err(Mishap::NotEnoughIotas(2, state.stack.len()));
    }

    let arg_count = 1;
    let iota = state.stack.get_iota::<NumberIota>(0, arg_count)?.int(0)?;
    state.stack.remove_args(&arg_count);

    if state.stack.len() < iota as usize {
        return Err(Mishap::NotEnoughIotas(iota as usize, state.stack.len()));
    }

    if iota >= 0 {
        let iota = iota as usize;
        let operation_result = { state.stack[state.stack.len() - iota].clone() };

        state.stack.remove(state.stack.len() - iota);

        state.stack.push_back(operation_result);
    } else {
        let arg_count = 1;
        let iota2 = state.stack.get_any_iota(0, arg_count)?.clone();
        state.stack.remove_args(&arg_count);

        state.stack.insert(iota.unsigned_abs() as usize, iota2)
    }

    Ok(state)
}

pub fn fisherman_copy<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    if state.stack.len() < 2 {
        return Err(Mishap::NotEnoughIotas(2, state.stack.len()));
    }

    let arg_count = 1;
    let iota = state.stack.get_iota::<NumberIota>(0, arg_count)?.int(0)? as usize;
    state.stack.remove_args(&arg_count);

    if state.stack.len() < iota {
        return Err(Mishap::NotEnoughIotas(iota, state.stack.len()));
    }

    let operation_result = { state.stack[state.stack.len() - 1 - iota].clone() };

    state.stack.push_back(operation_result);

    Ok(state)
}


pub fn mask<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
    value: Option<&ActionValue>,
) -> Result<&'a mut State, Mishap> {
    let code = match value {
        Some(ActionValue::Bookkeeper(code)) => code,
        Some(val) => Err(Mishap::InvalidValue(
            "Bookeeper Code".to_string(),
            format!("{:?}", val),
        ))?,
        None => Err(Mishap::ExpectedValue(
            "Bookkeeper's Gambit".to_string(),
            "bookkeeper Code".to_string(),
        ))?,
    };

    let apply_code = |iota, char| match char {
        '-' => Some(iota),
        'v' => None,
        _ => unreachable!(),
    };

    if state.stack.len() < code.len() {
        return Err(Mishap::NotEnoughIotas(code.len(), state.stack.len()));
    }

    let mut new_stack = state.stack.slice(..state.stack.len() - code.len());
    let top_stack = state.stack.slice(state.stack.len() - code.len()..);
    let apply_result = top_stack
        .iter()
        .zip(code.chars())
        .filter_map(|(i, char)| apply_code(i.clone(), char))
        .collect::<Vector<_>>();

    new_stack.append(apply_result);

    state.stack = new_stack;

    Ok(state)
}

// #[cfg(test)]
// mod tests {

//     use crate::pattern_registry::PatternRegistryExt;

//     use super::*;

//     #[test]
//     fn rotate_test() {
//         let mut state = State::default();
//         state.stack = vector![Iota::Number(0.0), Iota::Number(1.0), Iota::Number(2.0)];

//         let expected = vec![Iota::Number(1.0), Iota::Number(2.0), Iota::Number(0.0)];

//         let result = rotate(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }

//     #[test]
//     fn rotate_reverse_test() {
//         let mut state = State::default();
//         state.stack = vec![Iota::Number(0.0), Iota::Number(1.0), Iota::Number(2.0)];

//         let expected = vec![Iota::Number(2.0), Iota::Number(0.0), Iota::Number(1.0)];

//         let result = rotate_reverse(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }

//     #[test]
//     fn over_test() {
//         let mut state = State::default();
//         state.stack = vec![Iota::Number(0.0), Iota::Number(1.0)];

//         let expected = vec![Iota::Number(0.0), Iota::Number(1.0), Iota::Number(0.0)];

//         let result = over(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }

//     #[test]
//     fn tuck_test() {
//         let mut state = State::default();
//         state.stack = vec![Iota::Number(0.0), Iota::Number(1.0)];

//         let expected = vec![Iota::Number(1.0), Iota::Number(0.0), Iota::Number(1.0)];

//         let result = tuck(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }

//     #[test]
//     fn fisherman_test() {
//         let mut state = State::default();
//         state.stack = vec![
//             Iota::Number(0.0),
//             Iota::Number(1.0),
//             Iota::Number(2.0),
//             Iota::Number(2.0),
//         ];

//         let expected = vec![Iota::Number(0.0), Iota::Number(2.0), Iota::Number(1.0)];

//         let result = fisherman(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }

//     #[test]
//     fn fisherman_neg_test() {
//         todo!("negative fisherman isn't a thing yet")
//     }

//     #[test]
//     fn fisherman_copy_test() {
//         let mut state = State::default();
//         state.stack = vec![
//             Iota::Number(0.0),
//             Iota::Number(1.0),
//             Iota::Number(2.0),
//             Iota::Number(2.0),
//         ];

//         let expected = vec![
//             Iota::Number(0.0),
//             Iota::Number(1.0),
//             Iota::Number(2.0),
//             Iota::Number(0.0),
//         ];

//         let result = fisherman_copy(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }
// }

