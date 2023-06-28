use std::collections::HashMap;

use crate::{iota::Iota, parser::OpValue};

use super::state::State;

pub fn store<'a>(
    value: &'a Option<OpValue>,
    mut state: &'a mut State,
    heap: &'a mut HashMap<String, i32>,
    copy: bool,
) -> Result<(), String> {
    match value {
        Some(val) => match val {
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

                let (ravenmind, index) = insert_iota_into_ravenmind(state.ravenmind.clone(), iota);
                state.ravenmind = ravenmind;

                heap.insert(var.to_string(), index);
                Ok(())
            }
        },
        None => Err(format!("Expected 1 input, but recieved 0 inputs")),
    }
}

fn insert_iota_into_ravenmind(ravenmind: Option<Iota>, iota: Iota) -> (Option<Iota>, i32) {
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

pub fn push<'a>(
    value: &'a Option<OpValue>,
    state: &'a mut State,
    heap: &'a mut HashMap<String, i32>,
) -> Result<(), String> {
    match value {
        Some(val) => match val {
            OpValue::Iota(iota) => Err(format!("Expected Var, recieved {:?}", iota)),
            OpValue::Var(var) => {
                let index = *heap.get(var).ok_or("variable not assigned")?;
                let iota =
                    get_iota_from_ravenmind(state.ravenmind.clone(), index.try_into().unwrap())
                        .ok_or("no iota found at index")?;
                state.stack.push(iota);
                Ok(())
            }
        },
        None => Err(format!("Expected 1 input, but recieved 0 inputs")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut state = State {
            stack: vec![Iota::Number(1.0)],
            ravenmind: None,
        };
        let mut heap: HashMap<String, i32> = HashMap::new();
        let val = Some(OpValue::Var("$hello".to_string()));
        store(&val, &mut state, &mut heap, false).unwrap();
        println!("{:?}, {:?}", state, heap);
    }
}
