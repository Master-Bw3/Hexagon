use std::rc::Rc;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{EntityType, Stack, StackExt, State},
    },
    iota::{
        hex_casting::{
            entity::EntityIota,
            list::{ListIota, ListIotaExt},
            vector::VectorIota,
        },
        Iota,
    },
    parser::ActionValue,
    pattern_registry::PatternRegistry,
    patterns::{ActionNoValueType, ActionWithValueType},
};

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

pub fn spell_1<T: Iota>() -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        state.stack.get_iota::<T>(0, 1)?;
        state.stack.remove_args(&1);
        Ok(state)
    })
}

pub fn spell_2<T: Iota, U: Iota>() -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        state.stack.get_iota::<T>(0, 1)?;
        state.stack.get_iota::<U>(0, 1)?;

        state.stack.remove_args(&2);
        Ok(state)
    })
}

pub fn spell_3<T: Iota, U: Iota, V: Iota>() -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        state.stack.get_iota::<T>(0, 1)?;
        state.stack.get_iota::<U>(0, 1)?;
        state.stack.get_iota::<V>(0, 1)?;

        state.stack.remove_args(&3);
        Ok(state)
    })
}

pub fn value_0<U: Iota + 'static>(
    getter_type: &'static str,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early with an error if iota is of an invalid type
                    let iota = iota.clone().downcast_rc::<U>().map_err(|_| {
                        Mishap::InvalidValue(getter_type.to_string(), iota.display())
                    })?;

                    state.stack.push_back(iota)
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

pub fn value_1<T: Iota + 'static, U: Iota + 'static>(
    getter_type: &'static str,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            state.stack.get_iota::<T>(0, 2)?;
            state.stack.remove_args(&1);

            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early with an error if iota is of an invalid type
                    let iota = iota.clone().downcast_rc::<U>().map_err(|_| {
                        Mishap::InvalidValue(getter_type.to_string(), iota.display())
                    })?;

                    state.stack.push_back(iota)
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

pub fn value_2<T: Iota, U: Iota, V: Iota>(
    getter_type: &'static str,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            state.stack.get_iota::<T>(0, 2)?;
            state.stack.get_iota::<U>(0, 2)?;
            state.stack.remove_args(&2);

            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early with an error if iota is of an invalid type
                    let iota = iota.clone().downcast_rc::<V>().map_err(|_| {
                        Mishap::InvalidValue(getter_type.to_string(), iota.display())
                    })?;

                    state.stack.push_back(iota)
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
            let _ = &state.stack.get_iota::<VectorIota>(0, arg_count)?;
            state.stack.remove_args(&arg_count);

            let entity_type_str = entity_type_as_str(entity_type);
            match value {
                Some(ActionValue::Iota(iota)) => {
                    if iota
                    .clone()
                        .downcast_rc::<EntityIota>()
                        .map(|e| e.is_of_type(entity_type, &state.entities))
                        .unwrap_or(false)
                    {
                        state.stack.push_back(iota.clone())
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
            let _ = &state.stack.get_iota::<VectorIota>(0, arg_count)?;
            let _ = &state.stack.get_iota::<VectorIota>(1, arg_count)?;
            state.stack.remove_args(&arg_count);

            let conditon = |iota: Rc<dyn Iota>| {
                if *inverse {
                    iota.downcast_ref::<ListIota>()
                        .map(|e| {
                            e.is_entity_list(None, &state.entities)
                                && !e.is_entity_list(entity_type, &state.entities)
                        })
                        .unwrap_or(false)
                } else {
                    iota.downcast_ref::<ListIota>()
                        .map(|e| e.is_entity_list(entity_type, &state.entities))
                        .unwrap_or(false)
                }
            };

            match value {
                Some(ActionValue::Iota(iota)) => {
                    if conditon(Rc::clone(iota)) {
                        state.stack.push_back(iota.clone())
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

pub fn push_const(iota: Rc<dyn Iota>) -> Box<ActionNoValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry| -> Result<&mut State, Mishap> {
            state.stack.push_back(iota.clone());
            Ok(state)
        },
    )
}
