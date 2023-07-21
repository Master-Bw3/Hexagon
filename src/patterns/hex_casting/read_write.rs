use std::rc::Rc;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Holding, StackExt, State},
    },
    iota::{
        hex_casting::{
            entity::EntityIota, list::ListIota, null::NullIota, pattern::PatternIota,
            vector::VectorIota,
        },
        Iota,
    },
    pattern_registry::PatternRegistry,
};

pub fn read_local<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    state
        .stack
        .push_back(state.ravenmind.clone().unwrap_or(Rc::new(NullIota::Null)));

    Ok(state)
}

pub fn write_local<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_any_iota(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    state.ravenmind = Some(iota);

    Ok(state)
}

pub fn erase<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let player = state.entities.get_mut("Caster").unwrap();

    player.holding = Box::new(match *player.holding {
        Holding::None => Holding::None,
        Holding::Focus(_) => Holding::Focus(None),
        Holding::Trinket(_) => Holding::Trinket(None),
        Holding::Artifact(_) => Holding::Artifact(None),
        Holding::Cypher(_) => Holding::Cypher(None),
    });

    Ok(state)
}

pub fn craft_trinket<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota::<EntityIota>(0, arg_count)?,
        state.stack.get_iota::<ListIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let player = state.entities.get_mut("Caster").unwrap();

    player.holding = match *player.holding {
        Holding::Trinket(None) => Box::new(Holding::Trinket(Some(iotas.1))),
        _ => Err(Mishap::HoldingIncorrectItem)?,
    };

    Ok(state)
}

pub fn craft_cypher<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota::<EntityIota>(0, arg_count)?,
        state.stack.get_iota::<ListIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let player = state.entities.get_mut("Caster").unwrap();

    player.holding = match *player.holding {
        Holding::Trinket(None) => Box::new(Holding::Cypher(Some(iotas.1))),
        _ => Err(Mishap::HoldingIncorrectItem)?,
    };

    Ok(state)
}

pub fn craft_artifact<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota::<EntityIota>(0, arg_count)?,
        state.stack.get_iota::<ListIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let player = state.entities.get_mut("Caster").unwrap();

    player.holding = match *player.holding {
        Holding::Trinket(None) => Box::new(Holding::Artifact(Some(iotas.1))),
        _ => Err(Mishap::HoldingIncorrectItem)?,
    };

    Ok(state)
}

pub fn read<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let player = state.entities.get_mut("Caster").unwrap();

    let operation_result = match player.holding.as_ref() {
        Holding::Focus(Some(iota)) => iota.clone(),
        _ => Rc::new(NullIota::Null),
    };

    state.stack.push_back(operation_result);

    Ok(state)
}

pub fn write<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_any_iota(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let player = state.entities.get_mut("Caster").unwrap();

    player.holding = match player.holding.as_ref() {
        Holding::Focus(_) => Box::new(Holding::Focus(Some(iota))),
        _ => Err(Mishap::HoldingIncorrectItem)?,
    };

    Ok(state)
}

pub fn readable<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let player = state.entities.get("Caster").unwrap();

    let operation_result = match player.holding.as_ref() {
        Holding::None => false,
        Holding::Focus(_) => true,
        Holding::Trinket(_) => false,
        Holding::Artifact(_) => false,
        Holding::Cypher(_) => false,
    };

    state.stack.push_back(Rc::new(operation_result));

    Ok(state)
}

pub fn writable<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let player = state.entities.get("Caster").unwrap();

    let operation_result = match player.holding.as_ref() {
        Holding::None => false,
        Holding::Focus(_) => true,
        Holding::Trinket(_) => false,
        Holding::Artifact(_) => false,
        Holding::Cypher(_) => false,
    };

    state.stack.push_back(Rc::new(operation_result));

    Ok(state)
}

pub fn akashic_read<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota::<VectorIota>(0, arg_count)?,
        state.stack.get_iota::<PatternIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let location = &[(iotas.0).x as i32, (iotas.0).y as i32, (iotas.0).z as i32];

    let null: Rc<dyn Iota> = Rc::new(NullIota::Null);

    let operation_result = match state.libraries.get(location) {
        Some(library) => library.get(&iotas.1.signature).unwrap_or(&null).clone(),
        None => Err(Mishap::NoAkashicRecord(iotas.0))?,
    };

    state.stack.push_back(operation_result);

    Ok(state)
}

pub fn akashic_write<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let iotas = (
        state.stack.get_iota::<VectorIota>(0, arg_count)?,
        (*state.stack.get_iota::<PatternIota>(1, arg_count)?).clone(),
        state.stack.get_any_iota(2, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    let location = &[(iotas.0).x as i32, (iotas.0).y as i32, (iotas.0).z as i32];

    match state.libraries.get_mut(location) {
        Some(library) => library.insert((iotas.1).signature, iotas.2),
        None => Err(Mishap::NoAkashicRecord(iotas.0))?,
    };

    Ok(state)
}

pub fn read_entity<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<EntityIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = match state.entities.get(iota.name.as_ref()) {
        Some(entity) => match *entity.holding.clone() {
            Holding::Focus(iota) => iota.unwrap_or(Rc::new(NullIota::Null)),
            _ => todo!("handle unreadable item"),
        },
        None => todo!("handle entity not existing"),
    };

    state.stack.push_back(operation_result);

    Ok(state)
}

pub fn write_entity<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota::<EntityIota>(0, arg_count)?,
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    match state.entities.get_mut((iotas.0).name.as_ref()) {
        Some(entity) => match entity.holding.as_ref() {
            Holding::Focus(_) => entity.holding = Box::new(Holding::Focus(Some(iotas.1))),
            _ => todo!("handle unreadable item"),
        },
        None => todo!("handle entity not existing"),
    };

    Ok(state)
}

pub fn readable_entity<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<EntityIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = match state.entities.get(iota.name.as_ref()) {
        Some(entity) => match *entity.holding {
            Holding::None => false,
            Holding::Focus(_) => true,
            Holding::Trinket(_) => false,
            Holding::Artifact(_) => false,
            Holding::Cypher(_) => false,
        },
        None => todo!("handle entity not existing"),
    };

    state.stack.push_back(Rc::new(operation_result));

    Ok(state)
}

pub fn writeable_entity<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<EntityIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = match state.entities.get(iota.name.as_ref()) {
        Some(entity) => match *entity.holding {
            Holding::None => false,
            Holding::Focus(_) => true,
            Holding::Trinket(_) => false,
            Holding::Artifact(_) => false,
            Holding::Cypher(_) => false,
        },
        None => todo!("handle entity not existing"),
    };

    state.stack.push_back(Rc::new(operation_result));

    Ok(state)
}
