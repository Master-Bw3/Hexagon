use std::collections::HashMap;

use crate::{iota::Iota, parser::OpValue};

use super::state::State;

fn store<'a>(
    value: &'a OpValue,
    mut state: &'a mut State,
    heap: &'a mut HashMap<String, i32>,
    copy: bool,
) -> Result<(), String> {
    match value {
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
    }
}

//mutates ravenmind
fn insert_iota_into_ravenmind(ravenmind: Option<Iota>, iota: Iota) -> (Option<Iota>, i32) {
    let unwrapped_ravenmind: &mut Vec<Iota> = &mut match ravenmind {
        Some(Iota::List(list)) => list.clone(),
        _ => Vec::new(),
    };
    let index = unwrapped_ravenmind.len();
    unwrapped_ravenmind.push(iota);

    (Some(Iota::List(unwrapped_ravenmind.clone())), index.try_into().unwrap())
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
        let val = OpValue::Var("$hello".to_string());
        store(&val, &mut state, &mut heap, false).unwrap();
        println!("{:?}, {:?}", state, heap);

    }
}
