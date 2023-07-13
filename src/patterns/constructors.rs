use std::collections::HashMap;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Entity, EntityType, Stack, StackExt, State},
    },
    iota::{Display, Iota},
    parser::ActionValue,
    pattern_registry::PatternRegistry,
};

use super::{ActionNoValueType, ActionWithValueType};

type GetterType<T> = fn(&Stack, usize, usize) -> Result<T, Mishap>;

// pub fn operator<T: 'static>(arg_count: usize, action: &'static ActionNoValueType) -> Box<ActionNoValueType> {
//     Box::new(move |state: &mut State, pattern_registry: &PatternRegistry| {
//         let stack_len = state.stack.len();

//         if stack_len < arg_count {
//             Err(Mishap::NotEnoughIotas(arg_count - stack_len))?}

//         let args = &state.stack[stack_len-arg_count..];
//         state.stack.remove_args(&1);
//         let new_data = action(state, pattern_registry);

//         state.stack

//         todo!()

//     })
// }

pub fn spell_1<T: 'static>(getter: GetterType<T>) -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        getter(&state.stack, 0, 1)?;
        state.stack.remove_args(&1);
        Ok(state)
    })
}

pub fn spell_2<T: 'static, U: 'static>(
    getter1: GetterType<T>,
    getter2: GetterType<U>,
) -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        getter1(&state.stack, 0, 2)?;
        getter2(&state.stack, 1, 2)?;

        state.stack.remove_args(&2);
        Ok(state)
    })
}

pub fn spell_3<T: 'static, U: 'static, V: 'static>(
    getter1: GetterType<T>,
    getter2: GetterType<U>,
    getter3: GetterType<V>,
) -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        getter1(&state.stack, 0, 3)?;
        getter2(&state.stack, 1, 3)?;
        getter3(&state.stack, 2, 3)?;

        state.stack.remove_args(&3);
        Ok(state)
    })
}

pub fn value_0<U: Display + 'static>(
    value_type_getter: GetterType<U>,
    getter_type: &'static str,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early with an error if iota is of an invalid type
                    value_type_getter(&vec![iota.clone()], 0, 1).map_err(|_| {
                        Mishap::InvalidValue(getter_type.to_string(), iota.display())
                    })?;

                    state.stack.push(iota.clone())
                }
                Some(ActionValue::Bookkeeper(val)) => {
                    Err(Mishap::InvalidValue(getter_type.to_string(), val.clone()))?
                }
                None => Err(Mishap::ExpectedValue(
                    display_name.to_string(),
                    getter_type.to_string(),
                ))?,
            }

            Ok(state)
        },
    )
}

pub fn value_1<T: 'static, U: Display + 'static>(
    getter: GetterType<T>,
    value_type_getter: GetterType<U>,
    getter_type: &'static str,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            getter(&state.stack, 0, 1)?;
            state.stack.remove_args(&1);

            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early with an error if iota is of an invalid type
                    value_type_getter(&vec![iota.clone()], 0, 1).map_err(|_| {
                        Mishap::InvalidValue(getter_type.to_string(), iota.display())
                    })?;

                    state.stack.push(iota.clone())
                }
                Some(ActionValue::Bookkeeper(val)) => {
                    Err(Mishap::InvalidValue(getter_type.to_string(), val.clone()))?
                }
                None => Err(Mishap::ExpectedValue(
                    display_name.to_string(),
                    getter_type.to_string(),
                ))?,
            }

            Ok(state)
        },
    )
}

pub fn value_2<T: 'static, U: 'static, V: Display + 'static>(
    getter1: GetterType<T>,
    getter2: GetterType<U>,
    value_type_getter: GetterType<V>,
    getter_type: &'static str,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            getter1(&state.stack, 0, 2)?;
            getter2(&state.stack, 1, 2)?;
            state.stack.remove_args(&2);

            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early with an error if iota is of an invalid type
                    value_type_getter(&vec![iota.clone()], 0, 1).map_err(|_| {
                        Mishap::InvalidValue(getter_type.to_string(), iota.display())
                    })?;

                    state.stack.push(iota.clone())
                }
                Some(ActionValue::Bookkeeper(val)) => {
                    Err(Mishap::InvalidValue(getter_type.to_string(), val.clone()))?
                }
                None => Err(Mishap::ExpectedValue(
                    display_name.to_string(),
                    getter_type.to_string(),
                ))?,
            }

            Ok(state)
        },
    )
}

fn entity_type_as_str(entity_type: Option<&'static EntityType>) -> String {
    entity_type.map_or("Any".to_string(), |t| t.display())
}

pub fn get_entity(
    entity_type: Option<&'static EntityType>,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            let arg_count = 1;
            let _ = &state.stack.get_vector(0, arg_count)?;
            state.stack.remove_args(&arg_count);

            let entity_type_str = entity_type_as_str(entity_type);
            match value {
                Some(ActionValue::Iota(iota)) => {
                    if iota.is_entity(entity_type, &state.entities) {
                        state.stack.push(iota.clone())
                    } else {
                        Err(Mishap::InvalidValue(
                            format!("Entity of type {}", entity_type_str),
                            iota.display(),
                        ))?
                    }
                }
                Some(ActionValue::Bookkeeper(val)) => Err(Mishap::InvalidValue(
                    format!("Entity of type {}", entity_type_str),
                    val.clone(),
                ))?,
                None => Err(Mishap::ExpectedValue(
                    display_name.to_string(),
                    format!("Entity of type {}", entity_type_str),
                ))?,
            }

            Ok(state)
        },
    )
}

pub fn zone_entity(
    entity_type: Option<&'static EntityType>,
    inverse: &'static bool,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            let arg_count = 2;
            let _ = &state.stack.get_vector(0, arg_count)?;
            let _ = &state.stack.get_number(1, arg_count)?;
            state.stack.remove_args(&arg_count);

            let conditon = |iota: &Iota| {
                if *inverse {
                    iota.is_entity_list(None, &state.entities)
                        && !iota.is_entity_list(entity_type, &state.entities)
                } else {
                    iota.is_entity_list(entity_type, &state.entities)
                }
            };

            match value {
                Some(ActionValue::Iota(iota)) => {
                    if conditon(iota) {
                        state.stack.push(iota.clone())
                    } else {
                        Err(Mishap::InvalidValue(
                            format!(
                                "List of Entities of type {}",
                                entity_type.map_or("Any".to_string(), |t| t.display())
                            ),
                            iota.display(),
                        ))?
                    }
                }
                Some(ActionValue::Bookkeeper(val)) => Err(Mishap::InvalidValue(
                    format!(
                        "List of Entities of type {}",
                        entity_type.map_or("Any".to_string(), |t| t.display())
                    ),
                    val.clone(),
                ))?,
                None => Err(Mishap::ExpectedValue(
                    display_name.to_string(),
                    format!(
                        "List of Entities of type {}",
                        entity_type.map_or("Any".to_string(), |t| t.display())
                    ),
                ))?,
            }

            Ok(state)
        },
    )
}

pub fn push_const(iota: Iota) -> Box<ActionNoValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry| -> Result<&mut State, Mishap> {
            state.stack.push(iota.clone());
            Ok(state)
        },
    )
}
