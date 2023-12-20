use std::{collections::HashMap, rc::Rc, vec};

use crate::{
    interpreter::{mishap::Mishap, ops::EmbedType},
    iota::{hex_casting::pattern::PatternIota, Iota},
    parser::{ActionValue, Location, OpValue},
    pattern_registry::PatternRegistry,
};

pub fn compile_op_copy(
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let mut compiled: Vec<Rc<dyn Iota>> = vec![Rc::new(
        PatternIota::from_name(pattern_registry, "duplicate", None, Location::Unknown).unwrap(),
    )];

    compiled.append(&mut compile_op_store(heap, pattern_registry, arg)?);

    Ok(compiled)
}

pub fn compile_op_store(
    heap: &mut HashMap<String, i32>,
    registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;
    let (index, var) = {
        match value {
            OpValue::Iota(iota) => Err(Mishap::OpExpectedVar(iota.clone()))?,
            OpValue::Var(var) => (heap.get(var).copied(), var),
        }
    };

    let index = index.unwrap_or_else(|| {
        let new_index = heap.values().len() as i32;
        heap.insert(var.clone(), new_index);
        new_index
    });

    let compiled: Vec<Rc<dyn Iota>> = vec![
        Rc::new(PatternIota::from_name(registry, "read/local", None, Location::Unknown).unwrap()),
        Rc::new(
            PatternIota::from_name(
                registry,
                "number",
                Some(ActionValue::Iota(Rc::new(index as f64))),
                Location::Unknown,
            )
            .unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "rotate", None, Location::Unknown).unwrap()),
        Rc::new(
            PatternIota::from_name(registry, "modify_in_place", None, Location::Unknown).unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "write/local", None, Location::Unknown).unwrap()),
    ];

    Ok(compiled)
}

pub fn compile_op_push(
    heap: &mut HashMap<String, i32>,
    registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;

    let index = {
        match value {
            OpValue::Iota(iota) => Err(Mishap::OpExpectedVar(iota.clone()))?,
            OpValue::Var(var) => heap
                .get(var)
                .ok_or(Mishap::VariableNotAssigned(var.clone()))?,
        }
    };
    let compiled: Vec<Rc<dyn Iota>> = vec![
        Rc::new(PatternIota::from_name(registry, "read/local", None, Location::Unknown).unwrap()),
        Rc::new(
            PatternIota::from_name(
                registry,
                "number",
                Some(ActionValue::Iota(Rc::new(*index as f64))),
                Location::Unknown,
            )
            .unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "index", None, Location::Unknown).unwrap()),
    ];

    Ok(compiled)
}

pub fn compile_op_embed(
    registry: &PatternRegistry,
    depth: u32,
    arg: &Option<OpValue>,
    embed_type: EmbedType,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;

    let iota = {
        if let OpValue::Iota(iota) = value {
            iota.clone()
        } else {
            Err(Mishap::OpExpectedIota)?
        }
    };

    //handle smart embed
    let embed_type = match embed_type {
        EmbedType::Smart => {
            if depth > 0 {
                EmbedType::IntroRetro
            } else {
                EmbedType::Consider
            }
        }
        _ => embed_type,
    };

    let compiled = match embed_type {
        EmbedType::Normal => vec![iota],
        EmbedType::Consider => {
            let consideration = Rc::new(
                PatternIota::from_name(registry, "escape", None, Location::Unknown).unwrap(),
            );

            let mut result: Vec<Rc<dyn Iota>> = vec![consideration; i32::pow(2, depth) as usize];
            result.push(iota);
            result
        }
        EmbedType::IntroRetro => vec![
            Rc::new(
                PatternIota::from_name(registry, "open_paren", None, Location::Unknown).unwrap(),
            ),
            iota,
            Rc::new(
                PatternIota::from_name(registry, "close_paren", None, Location::Unknown).unwrap(),
            ),
            Rc::new(PatternIota::from_name(registry, "splat", None, Location::Unknown).unwrap()),
        ],
        EmbedType::Smart => unreachable!(),
    };

    Ok(compiled)
}
