use std::rc::Rc;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{EntityType, StackExt, State},
    },
    iota::{
        hex_casting::{
            entity::EntityIota,
            list::{ListIota, ListIotaExt},
            null::NullIota,
            vector::VectorIota,
        },
        Iota,
    },
    parser::ActionValue,
    pattern_registry::PatternRegistry,
    patterns::{ActionNoValueType, ActionWithValueType},
};

pub fn spell_1<T: Iota>() -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        state.stack.get_iota::<T>(0, 1)?;
        state.stack.remove_args(&1);
        Ok(state)
    })
}

pub fn spell_2<T: Iota, U: Iota>() -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        state.stack.get_iota::<T>(0, 2)?;
        state.stack.get_iota::<U>(1, 2)?;

        state.stack.remove_args(&2);
        Ok(state)
    })
}

pub fn spell_3<T: Iota, U: Iota, V: Iota>() -> Box<ActionNoValueType> {
    Box::new(move |state: &mut State, _: &PatternRegistry| {
        state.stack.get_iota::<T>(0, 3)?;
        state.stack.get_iota::<U>(1, 3)?;
        state.stack.get_iota::<V>(2, 3)?;

        state.stack.remove_args(&3);
        Ok(state)
    })
}

pub fn value_0<T: Iota + 'static>(
    getter_type: &'static str,
    accept_null: bool,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early if iota is null and action accepts null
                    if accept_null && iota.clone().downcast_rc::<NullIota>().is_ok() {
                        state.stack.push_back(iota.clone());
                        return Ok(state);
                    }

                    //return early with an error if iota is of an invalid type
                    iota.clone()
                        .downcast_rc::<T>()
                        .map_err(|_| Mishap::InvalidValue {
                            expected: getter_type.to_string(),
                            received: iota.display(),
                        })?;

                    state.stack.push_back(iota.clone());
                    Ok(state)
                }
                Some(ActionValue::Bookkeeper(val)) => Err(Mishap::InvalidValue {
                    expected: getter_type.to_string(),
                    received: val.clone(),
                })?,
                None => Err(Mishap::ExpectedValue {
                    caused_by: display_name.to_string(),
                    expected: getter_type.to_string(),
                })?,
            }
        },
    )
}

pub fn value_1<T: Iota + 'static, U: Iota + 'static>(
    getter_type: &'static str,
    accept_null: bool,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            state.stack.get_iota::<T>(0, 1)?;
            state.stack.remove_args(&1);

            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early if iota is null and action accepts null
                    if accept_null && iota.clone().downcast_rc::<NullIota>().is_ok() {
                        state.stack.push_back(iota.clone());
                        return Ok(state);
                    }

                    //return early with an error if iota is of an invalid type
                    iota.clone()
                        .downcast_rc::<U>()
                        .map_err(|_| Mishap::InvalidValue {
                            expected: getter_type.to_string(),
                            received: iota.display(),
                        })?;

                    state.stack.push_back(iota.clone());
                    Ok(state)
                }
                Some(ActionValue::Bookkeeper(val)) => Err(Mishap::InvalidValue {
                    expected: getter_type.to_string(),
                    received: val.clone(),
                })?,
                None => Err(Mishap::ExpectedValue {
                    caused_by: display_name.to_string(),
                    expected: getter_type.to_string(),
                })?,
            }
        },
    )
}

pub fn value_2<T: Iota, U: Iota, V: Iota>(
    getter_type: &'static str,
    accept_null: bool,
    display_name: &'static str,
) -> Box<ActionWithValueType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry, value: Option<&ActionValue>| {
            state.stack.get_iota::<T>(0, 2)?;
            state.stack.get_iota::<U>(1, 2)?;
            state.stack.remove_args(&2);

            match value {
                Some(ActionValue::Iota(iota)) => {
                    //return early if iota is null and action accepts null
                    if accept_null && iota.clone().downcast_rc::<NullIota>().is_ok() {
                        state.stack.push_back(iota.clone());
                        return Ok(state);
                    }

                    //return early with an error if iota is of an invalid type
                    iota.clone()
                        .downcast_rc::<V>()
                        .map_err(|_| Mishap::InvalidValue {
                            expected: getter_type.to_string(),
                            received: iota.display(),
                        })?;

                    state.stack.push_back(iota.clone());
                    Ok(state)
                }
                Some(ActionValue::Bookkeeper(val)) => Err(Mishap::InvalidValue {
                    expected: getter_type.to_string(),
                    received: val.clone(),
                })?,
                None => Err(Mishap::ExpectedValue {
                    caused_by: display_name.to_string(),
                    expected: getter_type.to_string(),
                })?,
            }
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
                        Err(Mishap::InvalidValue {
                            expected: format!("Entity of type {}", entity_type_str),
                            received: iota.display(),
                        })?
                    }
                }
                Some(ActionValue::Bookkeeper(val)) => Err(Mishap::InvalidValue {
                    expected: format!("Entity of type {}", entity_type_str),
                    received: val.clone(),
                })?,
                None => Err(Mishap::ExpectedValue {
                    caused_by: display_name.to_string(),
                    expected: format!("Entity of type {}", entity_type_str),
                })?,
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
                        Err(Mishap::InvalidValue {
                            expected: format!(
                                "List of Entities of type {}",
                                entity_type.map_or("Any".to_string(), |t| t.display())
                            ),
                            received: iota.display(),
                        })?
                    }
                }
                Some(ActionValue::Bookkeeper(val)) => Err(Mishap::InvalidValue {
                    expected: format!(
                        "List of Entities of type {}",
                        entity_type.map_or("Any".to_string(), |t| t.display())
                    ),
                    received: val.clone(),
                })?,
                None => Err(Mishap::ExpectedValue {
                    caused_by: display_name.to_string(),
                    expected: format!(
                        "List of Entities of type {}",
                        entity_type.map_or("Any".to_string(), |t| t.display())
                    ),
                })?,
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
