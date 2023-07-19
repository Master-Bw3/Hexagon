use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Holding, StackExt, State},
    },
    iota::{Iota, hex_casting::null::NullIota},
    pattern_registry::PatternRegistry,
};

pub fn read_local<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    state.stack.push(
        state
            .ravenmind
            .clone()
            .unwrap_or(Iota::Null(NullIota::Null)),
    );

    Ok(state)
}

pub fn write_local<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota(0, arg_count)?.clone();
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
        state.stack.get_entity(0, arg_count)?,
        state.stack.get_list(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let player = state.entities.get_mut("Caster").unwrap();

    player.holding = match *player.holding {
        Holding::Trinket(None) => Box::new(Holding::Trinket(Some(Iota::List(iotas.1)))),
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
        state.stack.get_entity(0, arg_count)?,
        state.stack.get_list(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let player = state.entities.get_mut("Caster").unwrap();

    player.holding = match *player.holding {
        Holding::Trinket(None) => Box::new(Holding::Cypher(Some(Iota::List(iotas.1)))),
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
        state.stack.get_entity(0, arg_count)?,
        state.stack.get_list(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let player = state.entities.get_mut("Caster").unwrap();

    player.holding = match *player.holding {
        Holding::Trinket(None) => Box::new(Holding::Artifact(Some(Iota::List(iotas.1)))),
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
        _ => Iota::Null(NullIota::Null),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn write<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota(0, arg_count)?.clone();
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
        Holding::None => Iota::Bool(false),
        Holding::Focus(_) => Iota::Bool(true),
        Holding::Trinket(_) => Iota::Bool(false),
        Holding::Artifact(_) => Iota::Bool(false),
        Holding::Cypher(_) => Iota::Bool(false),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn writable<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let player = state.entities.get("Caster").unwrap();

    let operation_result = match player.holding.as_ref() {
        Holding::None => Iota::Bool(false),
        Holding::Focus(_) => Iota::Bool(true),
        Holding::Trinket(_) => Iota::Bool(false),
        Holding::Artifact(_) => Iota::Bool(false),
        Holding::Cypher(_) => Iota::Bool(false),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn akashic_read<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_vector(0, arg_count)?,
        state.stack.get_pattern(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let location = &[(iotas.0).x as i32, (iotas.0).y as i32, (iotas.0).z as i32];

    let operation_result = match state.libraries.get(location) {
        Some(library) => library
            .get(&iotas.1.signature)
            .unwrap_or(&Iota::Null(NullIota::Null))
            .clone(),
        None => Err(Mishap::NoAkashicRecord(iotas.0))?,
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn akashic_write<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let iotas = (
        state.stack.get_vector(0, arg_count)?,
        state.stack.get_pattern(1, arg_count)?,
        state.stack.get_iota(2, arg_count)?.clone(),
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
    let iota = state.stack.get_entity(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = match state.entities.get(&iota) {
        Some(entity) => match *entity.holding.clone() {
            Holding::Focus(iota) => iota.unwrap_or(Iota::Null(NullIota::Null)),
            _ => todo!("handle unreadable item"),
        },
        None => todo!("handle entity not existing"),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn write_entity<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_entity(0, arg_count)?,
        state.stack.get_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    match state.entities.get_mut(&(iotas.0)) {
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
    let iota = state.stack.get_entity(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = match state.entities.get(&iota) {
        Some(entity) => match *entity.holding {
            Holding::None => Iota::Bool(false),
            Holding::Focus(_) => Iota::Bool(true),
            Holding::Trinket(_) => Iota::Bool(false),
            Holding::Artifact(_) => Iota::Bool(false),
            Holding::Cypher(_) => Iota::Bool(false),
        },
        None => todo!("handle entity not existing"),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn writeable_entity<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_entity(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = match state.entities.get(&iota) {
        Some(entity) => match *entity.holding {
            Holding::None => Iota::Bool(false),
            Holding::Focus(_) => Iota::Bool(true),
            Holding::Trinket(_) => Iota::Bool(false),
            Holding::Artifact(_) => Iota::Bool(false),
            Holding::Cypher(_) => Iota::Bool(false),
        },
        None => todo!("handle entity not existing"),
    };

    state.stack.push(operation_result);

    Ok(state)
}
