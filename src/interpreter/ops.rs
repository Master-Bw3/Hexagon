use std::{ops::Deref, rc::Rc};

use im::Vector;

use crate::{
    iota::{
        hex_casting::{
            list::ListIota,
            pattern::{PatternIota, SignatureExt},
        },
        Iota,
    },
    parser::OpValue,
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use super::{interpret_action, mishap::Mishap, push_iota, state::State};

pub fn store<'a>(
    value: &'a Option<OpValue>,
    state: &'a mut State,
    copy: bool,
) -> Result<(), Mishap> {
    let val = match value {
        Some(val) => val,
        None => Err(Mishap::OpNotEnoughArgs(1))?,
    };
    match val {
        OpValue::Iota(iota) => Err(Mishap::OpExpectedVar(iota.clone())),
        OpValue::Var(var) => {
            let iota = {
                if copy {
                    state
                        .stack
                        .last()
                        .ok_or(Mishap::NotEnoughIotas(1, state.stack.len()))?
                        .clone()
                } else {
                    state
                        .stack
                        .pop_back()
                        .ok_or(Mishap::NotEnoughIotas(1, state.stack.len()))?
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
    ravenmind: Option<Rc<dyn Iota>>,
    iota: Rc<dyn Iota>,
    index: usize,
) -> (Option<Rc<dyn Iota>>, i32) {
    let unwrapped_ravenmind: Rc<ListIota> = match ravenmind {
        Some(ref list) => list
            .clone()
            .downcast_rc::<ListIota>()
            .unwrap_or(Rc::new(Vector::new())),
        _ => Rc::new(Vector::new()),
    };
    let mut unwrapped_ravenmind = unwrapped_ravenmind.deref().clone();

    if unwrapped_ravenmind.len() > index {
        unwrapped_ravenmind.remove(index);
        unwrapped_ravenmind.insert(index, iota);
        (
            Some(Rc::new(unwrapped_ravenmind)),
            index.try_into().unwrap(),
        )
    } else {
        add_iota_to_ravenmind(ravenmind, iota)
    }
}

fn add_iota_to_ravenmind(
    ravenmind: Option<Rc<dyn Iota>>,
    iota: Rc<dyn Iota>,
) -> (Option<Rc<dyn Iota>>, i32) {
    let unwrapped_ravenmind: &Rc<Vector<Rc<dyn Iota>>> = &mut match ravenmind {
        Some(r) => r
            .downcast_rc::<ListIota>()
            .unwrap_or(Rc::new(Vector::new())),
        _ => Rc::new(Vector::new()),
    };
    let mut unwrapped_ravenmind = unwrapped_ravenmind.deref().clone();

    let index = unwrapped_ravenmind.len();
    unwrapped_ravenmind.push_back(iota);

    (
        Some(Rc::new(unwrapped_ravenmind.clone())),
        index.try_into().unwrap(),
    )
}

fn get_iota_from_ravenmind(ravenmind: Option<Rc<dyn Iota>>, index: usize) -> Option<Rc<dyn Iota>> {
    let unwrapped_ravenmind: &mut Rc<ListIota> = &mut match ravenmind {
        Some(iota) => iota
            .downcast_rc::<ListIota>()
            .unwrap_or(Rc::new(Vector::new())),
        _ => Rc::new(Vector::new()),
    };

    unwrapped_ravenmind.get(index).cloned()
}

pub fn push<'a>(value: &'a Option<OpValue>, state: &'a mut State) -> Result<(), Mishap> {
    match value {
        Some(val) => match val {
            OpValue::Iota(iota) => Err(Mishap::OpExpectedVar(iota.clone()))?,
            OpValue::Var(var) => {
                let index = *state
                    .heap
                    .get(var)
                    .ok_or(Mishap::VariableNotAssigned(var.clone()))?
                    as usize;
                let iota = get_iota_from_ravenmind(state.ravenmind.clone(), index)
                    .ok_or(Mishap::NoIotaAtIndex(index))?;
                push_iota(iota, state, state.consider_next);
                state.consider_next = false;
                Ok(())
            }
        },
        None => Err(Mishap::OpNotEnoughArgs(1))?,
    }
}

pub enum EmbedType {
    Normal,
    Smart,
    Consider,
    IntroRetro,
}

pub fn embed<'a>(
    value: &'a Option<OpValue>,
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
    embed_type: EmbedType,
) -> Result<(), Mishap> {
    let val = match value {
        Some(val) => val,
        None => Err(Mishap::OpNotEnoughArgs(1))?,
    };

    match val {
        OpValue::Iota(iota) => match embed_type {
            EmbedType::Normal => match iota.clone().downcast_rc::<PatternIota>() {
                Ok(pat) => {
                    interpret_action(
                        pattern_registry
                            .find(&pat.signature.as_str(), &None)
                            .ok_or(Mishap::InvalidPattern)?
                            .internal_name,
                        *pat.value.clone(),
                        state,
                        pattern_registry,
                        None
                    )?;
                }
                _ => return Err(Mishap::ExpectedPattern(iota.clone())),
            },
            _ => state.stack.push_back(iota.clone()),
        },
        OpValue::Var(_) => Err(Mishap::OpExpectedIota)?,
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let mut state = State::default();
        let heap: HashMap<String, i32> = HashMap::new();
        let val = Some(OpValue::Var("$hello".to_string()));
        store(&val, &mut state, false).unwrap();
        println!("{:?}, {:?}", state.stack, heap);
    }
}
