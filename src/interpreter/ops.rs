use std::collections::HashMap;

use crate::{
    iota::{Iota, PatternIota, PatternIotaExt},
    parser::OpValue,
};

use super::{
    push_iota,
    state::{Stack, State},
};

pub fn store<'a>(
    value: &'a Option<OpValue>,
    mut state: &'a mut State,
    copy: bool,
) -> Result<(), String> {
    let val = match value {
        Some(val) => val,
        None => Err(format!("Expected 1 input, but recieved 0 inputs"))?,
    };
    match val {
        OpValue::Iota(iota) => Err(format!("Expected Var, recieved {:?}", iota)),
        OpValue::Var(var) => {
            let iota = {
                if copy {
                    state
                        .stack
                        .last()
                        .ok_or("Cannot assign variable because stack is empty".to_string())?
                        .clone()
                } else {
                    state
                        .stack
                        .pop()
                        .ok_or("Cannot assign variable because stack is empty".to_string())?
                }
            };

            let (ravenmind, index) = match state.heap.get(var) {
                Some(index) => insert_iota_into_ravenmind(
                    state.ravenmind.clone(),
                    iota,
                    (*index).try_into().unwrap(),
                ),
                None => add_iota_to_ravenmind(state.ravenmind.clone(), iota),
            };
            state.ravenmind = ravenmind;
            state.heap.insert(var.to_string(), index);
            Ok(())
        }
    }
}

fn insert_iota_into_ravenmind(
    ravenmind: Option<Iota>,
    iota: Iota,
    index: usize,
) -> (Option<Iota>, i32) {
    let mut unwrapped_ravenmind: Vec<Iota> = match ravenmind {
        Some(Iota::List(ref list)) => list.clone(),
        _ => Vec::new(),
    };

    if unwrapped_ravenmind.len() > index {
        unwrapped_ravenmind.remove(index);
        unwrapped_ravenmind.insert(index, iota);
        (
            Some(Iota::List(unwrapped_ravenmind)),
            index.try_into().unwrap(),
        )
    } else {
        add_iota_to_ravenmind(ravenmind, iota)
    }
}

fn add_iota_to_ravenmind(ravenmind: Option<Iota>, iota: Iota) -> (Option<Iota>, i32) {
    let unwrapped_ravenmind: &mut Vec<Iota> = &mut match ravenmind {
        Some(Iota::List(list)) => list.clone(),
        _ => Vec::new(),
    };
    let index = unwrapped_ravenmind.len();
    unwrapped_ravenmind.push(iota);

    (
        Some(Iota::List(unwrapped_ravenmind.clone())),
        index.try_into().unwrap(),
    )
}

fn get_iota_from_ravenmind(ravenmind: Option<Iota>, index: usize) -> Option<Iota> {
    let unwrapped_ravenmind: &mut Vec<Iota> = &mut match ravenmind {
        Some(Iota::List(list)) => list.clone(),
        _ => Vec::new(),
    };

    unwrapped_ravenmind.get(index).cloned()
}

pub fn push<'a>(value: &'a Option<OpValue>, state: &'a mut State) -> Result<(), String> {
    match value {
        Some(val) => match val {
            OpValue::Iota(iota) => Err(format!("Expected Var, recieved {:?}", iota)),
            OpValue::Var(var) => {
                let index = *state.heap.get(var).ok_or("variable not assigned")?;
                let iota =
                    get_iota_from_ravenmind(state.ravenmind.clone(), index.try_into().unwrap())
                        .ok_or("no iota found at index")?;
                push_iota(iota, state, state.consider_next);
                state.consider_next = false;
                Ok(())
            }
        },
        None => Err(format!("Expected 1 input, but recieved 0 inputs")),
    }
}

pub enum embedType {
    Normal,
    Smart,
    Consider,
    IntroRetro,
}

pub fn embed<'a>(
    value: &'a Option<OpValue>,
    mut state: &'a mut State,
    embed_type: embedType,
) -> Result<(), String> {
    let val = match value {
        Some(val) => val,
        None => Err(format!("Expected 1 input, but recieved 0 inputs"))?,
    };
    match val {
        OpValue::Iota(iota) => match embed_type {
            embedType::Normal => {
                state.stack.push(iota.clone());
            }
            embedType::Smart => todo!(),
            embedType::Consider => todo!(),
            embedType::IntroRetro => {
                state.stack.push(Iota::Pattern(PatternIota::from_name(
                    &state.pattern_registry,
                    "open_paren",
                )));
                state.stack.push(iota.clone());
            }
        },
        OpValue::Var(var) => Err(format!("Expected Iota, recieved {:?}", var))?,
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::pattern_registry::{PatternRegistry, PatternRegistryExt};

    use super::*;

    #[test]
    fn test() {
        let mut state = State {
            stack: vec![Iota::Number(1.0)],
            ravenmind: None,
            buffer: None,
            consider_next: false,
            pattern_registry: PatternRegistry::construct(),
            heap: HashMap::new(),
        };
        let mut heap: HashMap<String, i32> = HashMap::new();
        let val = Some(OpValue::Var("$hello".to_string()));
        store(&val, &mut state, false).unwrap();
        println!("{:?}, {:?}", state.stack, heap);
    }
}
