use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Holding, StackExt, State},
    },
    iota::{Iota, NullIota},
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
    state.offhand = match state.offhand {
        Holding::None => Holding::None,
        Holding::Focus(_) => Holding::Focus(None),
        Holding::Trinket(_) => Holding::Trinket(None),
        Holding::Artifact(_) => Holding::Artifact(None),
        Holding::Cypher(_) => Holding::Cypher(None),
    };

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

    state.offhand = match state.offhand {
        Holding::Trinket(None) => Holding::Trinket(Some(Iota::List(iotas.1))),
        _ => state.offhand.clone(), //should mishap but im lazy so no
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

    state.offhand = match state.offhand {
        Holding::Trinket(None) => Holding::Cypher(Some(Iota::List(iotas.1))),
        _ => state.offhand.clone(), //should mishap but im lazy so no
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

    state.offhand = match state.offhand {
        Holding::Trinket(None) => Holding::Artifact(Some(Iota::List(iotas.1))),
        _ => state.offhand.clone(), //should mishap but im lazy so no
    };

    Ok(state)
}

pub fn read<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let operation_result = match &state.offhand {
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

    state.offhand = match state.offhand {
        Holding::Focus(_) => Holding::Focus(Some(iota)),
        _ => state.offhand.clone(), //should mishap but im lazy so no
    };

    Ok(state)
}
