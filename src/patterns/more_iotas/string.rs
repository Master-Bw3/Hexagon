use std::rc::Rc;

use crate::interpreter::state::Either;
use crate::iota::hex_casting::bool::BooleanIota;
use crate::iota::hex_casting::garbage::GarbageIota;
use crate::iota::hex_casting::pattern::SignatureExt;
use crate::iota::hex_casting::{list::ListIota, pattern::PatternIota};
use crate::iota::more_iotas::string::StringVecExt;
use crate::pattern_registry::PatternRegistryExt;
use im::Vector;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{
        hex_casting::{
            null::NullIota,
            number::{NumberIota, NumberIotaExt},
            vector::VectorIota,
        },
        more_iotas::string::StringIota,
        Iota,
    },
    pattern_registry::PatternRegistry,
};

pub fn concat<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota::<StringIota>(0, arg_count)?,
        state.stack.get_iota::<StringIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let mut concatted = (*iotas.0).clone();
    concatted.push_str((iotas.1).as_ref());

    state.stack.push_back(Rc::new(concatted));

    Ok(state)
}

pub fn split<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;

    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    let delimiter = state.stack.get_iota::<StringIota>(1, arg_count)?;

    state.stack.remove_args(&arg_count);

    let strings = (*string)
        .clone()
        .split(delimiter.as_str())
        .map(|str| -> Rc<dyn Iota> { Rc::new(str.to_owned()) })
        .collect::<Vector<_>>();

    state.stack.push_back(Rc::new(strings));

    Ok(state)
}

pub fn parse<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let parse_result: Rc<dyn Iota> = match string.parse::<NumberIota>() {
        Ok(num) => Rc::new(num),
        Err(_) => Rc::new(NullIota),
    };

    state.stack.push_back(parse_result);

    Ok(state)
}

pub fn find<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    let query = state.stack.get_iota::<StringIota>(1, arg_count)?;
    state.stack.remove_args(&arg_count);

    let find_count = string.matches(query.as_str()).count() as NumberIota;

    state.stack.push_back(Rc::new(find_count));

    Ok(state)
}

pub fn sub<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    let start = state
        .stack
        .get_iota::<NumberIota>(1, arg_count)?
        .positive_int_under_inclusive(1, string.len())? as usize;
    let end = state
        .stack
        .get_iota::<NumberIota>(2, arg_count)?
        .positive_int_under_inclusive(2, string.len())? as usize;

    state.stack.remove_args(&arg_count);

    let find_count = string[start..end].to_owned();

    state.stack.push_back(Rc::new(find_count));

    Ok(state)
}

pub fn len<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let len = string.len() as NumberIota;

    state.stack.push_back(Rc::new(len));

    Ok(state)
}

pub fn write<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    state.stack.get_iota::<VectorIota>(0, arg_count)?;
    let iota = state.stack.get_any_iota(1, arg_count)?;

    //check for list of strings or a single string
    if matches!(
        iota.clone()
            .downcast_rc::<ListIota>()
            .map(|list| list.is_string_vec()),
        Ok(true)
    ) || iota.clone().downcast_rc::<StringIota>().is_ok()
    {
    } else {
        Err(Mishap::IncorrectIota {
            index: 0,
            expected: "String or List of Strings".to_string(),
            received: iota,
        })?
    }

    state.stack.remove_args(&arg_count);

    Ok(state)
}

pub fn set_prefix<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    //check for null or a string
    let iota = state.stack.get_any_iota(0, arg_count)?;

    if iota.clone().downcast_rc::<NullIota>().is_ok()
        || iota.clone().downcast_rc::<StringIota>().is_ok()
    {
        state.stack.remove_args(&arg_count);

        Ok(state)
    } else {
        Err(Mishap::IncorrectIota {
            index: 0,
            expected: "String or Null".to_string(),
            received: iota,
        })
    }
}

pub fn display_iota<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_any_iota(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    if let Ok(pattern) = iota.clone().downcast_rc::<PatternIota>() {
        //TODO: implement pattern directionality
        let formatted = format!("HexPattern(EAST {})", pattern.signature.as_str());
        state.stack.push_back(Rc::new(formatted));
    } else if iota.clone().downcast_rc::<GarbageIota>().is_ok() {
        state
            .stack
            .push_back(Rc::new("arimfexendrapuse".to_string()));
    } else {
        state.stack.push_back(Rc::new(iota.display()));
    }

    Ok(state)
}

pub fn display_action<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let pattern = state.stack.get_iota::<PatternIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let string: Rc<dyn Iota> =
        match pattern_registry.find(&pattern.signature.as_str(), &pattern.value) {
            Some(_) => Rc::new(pattern.display()),
            None => Rc::new(NullIota),
        };

    state.stack.push_back(string);

    Ok(state)
}

pub fn set_case<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let string = state.stack.get_iota::<String>(0, arg_count)?;
    let option = state
        .stack
        .get_iota_a_or_b::<BooleanIota, NullIota>(1, arg_count)?;
    state.stack.remove_args(&arg_count);

    let new_string = match option {
        Either::L(bool) => {
            if *bool {
                string.to_uppercase()
            } else {
                string.to_lowercase()
            }
        }
        Either::R(_null) => string
            .chars()
            .into_iter()
            .map(|char| {
                if char.is_lowercase() {
                    char.to_uppercase().next().unwrap_or(char)
                } else {
                    char.to_lowercase().next().unwrap_or(char)
                }
            })
            .collect(),
    };

    state.stack.push_back(Rc::new(new_string));

    Ok(state)
}
