use std::{collections::HashMap, vec};

use crate::{
    interpreter::{mishap::Mishap, ops::EmbedType},
    iota::{Iota, PatternIota},
    parser::OpValue,
    pattern_registry::PatternRegistry,
};

pub fn compile_op_copy(
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Iota>, Mishap> {
    let mut compiled = vec![Iota::Pattern(PatternIota::from_name(
        pattern_registry,
        "duplicate",
        None,
    ))];

    compiled.append(&mut compile_op_store(heap, pattern_registry, arg)?);

    Ok(compiled)
}

pub fn compile_op_store(
    heap: &mut HashMap<String, i32>,
    registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Iota>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;

    let (index, var) = {
        match value {
            OpValue::Iota(iota) => Err(Mishap::OpExpectedVar(iota.clone()))?,
            OpValue::Var(var) => (heap.get(var), var),
        }
    };

    let compiled = match index {
        Some(index) => {
            vec![
                Iota::Pattern(PatternIota::from_name(registry, "read/local", None)),
                Iota::Pattern(PatternIota::from_name(
                    registry,
                    "number",
                    Some(Iota::Number(*index as f32)),
                )),
                Iota::Pattern(PatternIota::from_name(registry, "rotate", None)),
                Iota::Pattern(PatternIota::from_name(registry, "modify_in_place", None)),
                Iota::Pattern(PatternIota::from_name(registry, "write/local", None)),
            ]
        }
        None => {
            let new_index = heap.values().fold(0, |acc, val| i32::max(acc, *val));
            heap.insert(var.clone(), new_index);

            vec![
                Iota::Pattern(PatternIota::from_name(registry, "read/local", None)),
                Iota::Pattern(PatternIota::from_name(registry, "swap", None)),
                Iota::Pattern(PatternIota::from_name(registry, "append", None)),
                Iota::Pattern(PatternIota::from_name(registry, "write/local", None)),
            ]
        }
    };

    Ok(compiled)
}

pub fn compile_op_push(
    heap: &mut HashMap<String, i32>,
    registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Iota>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;

    let index = {
        match value {
            OpValue::Iota(iota) => Err(Mishap::OpExpectedVar(iota.clone()))?,
            OpValue::Var(var) => heap.get(var).ok_or(Mishap::VariableNotAssigned)?,
        }
    };
    let compiled = vec![
        Iota::Pattern(PatternIota::from_name(registry, "read/local", None)),
        Iota::Pattern(PatternIota::from_name(
            registry,
            "number",
            Some(Iota::Number(*index as f32)),
        )),
        Iota::Pattern(PatternIota::from_name(registry, "index", None)),
    ];

    Ok(compiled)
}

pub fn compile_op_embed(
    registry: &PatternRegistry,
    buffer: &Option<Vec<(Iota, bool)>>,
    arg: &Option<OpValue>,
    embed_type: EmbedType,
) -> Result<Vec<Iota>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;

    let iota = {
        if let OpValue::Iota(iota) = value {
            iota.clone()
        } else {
            Err(Mishap::OpExpectedIota)?
        }
    };

    let intro_pattern = Iota::Pattern(PatternIota::from_name(registry, "open_paren", None));
    let intro_count: u32 = if let Some(inner_buffer) = buffer {
        inner_buffer.iter().fold(0, |acc, x| {
            if x.0 == intro_pattern && !x.1 {
                acc + 1
            } else {
                acc
            }
        })
    } else {
        0
    };

    //handle smart embed
    let embed_type = match embed_type {
        EmbedType::Smart => {
            if intro_count > 0 {
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
            let mut result = vec![
                Iota::Pattern(PatternIota::from_name(registry, "escape", None,));
                i32::pow(2, intro_count) as usize
            ];
            result.push(iota);
            result
        }
        EmbedType::IntroRetro => vec![
            Iota::Pattern(PatternIota::from_name(registry, "open_paren", None)),
            iota,
            Iota::Pattern(PatternIota::from_name(registry, "close_paren", None)),
            Iota::Pattern(PatternIota::from_name(registry, "splat", None)),
        ],
        EmbedType::Smart => unreachable!(),
    };

    Ok(compiled)
}
