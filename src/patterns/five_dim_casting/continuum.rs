use std::{cell::RefCell, rc::Rc};

use im::vector;

use crate::{
    interpreter::{
        continuation::{
            iota_list_to_ast_node_list, ContinuationFrame, FrameEvaluate, FrameIterate,
        },
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{
        five_dim_casting::continuum::ContinuumIota,
        hex_casting::{list::ListIota, null::NullIota, number::NumberIota, pattern::PatternIota},
        Iota,
    },
    parser::{ActionValue, AstNode},
    pattern_registry::PatternRegistry,
};

pub fn number_stream<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let one: Rc<dyn Iota> = Rc::new(
        PatternIota::from_name(
            pattern_registry,
            "number",
            Some(ActionValue::Iota(Rc::new(1.0))),
            None,
        )
        .unwrap(),
    );
    let add: Rc<dyn Iota> =
        Rc::new(PatternIota::from_name(pattern_registry, "add", None, None).unwrap());

    let continuum = ContinuumIota {
        front_val: Rc::new(0.0),
        gen_next_func: Rc::new(vector![one, add]),
        maps: vector![],
    };

    state.stack.push_back(Rc::new(continuum));

    Ok(state)
}

pub fn get<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    {
        let arg_count = 2;
        let iotas = (
            (*state.stack.get_iota::<ContinuumIota>(0, arg_count)?).clone(),
            state.stack.get_iota::<NumberIota>(1, arg_count)?,
        );
        state.stack.remove_args(&arg_count);

        state
            .continuation
            .push_back(ContinuationFrame::Iterate(FrameIterate {
                base_stack: None,
                index: 0,
                collect: (*iotas.1 as usize, *iotas.1 as usize),
                acc: Rc::new(RefCell::new(vector![])),
                prev: iotas.0.front_val,
                gen_next_code: iota_list_to_ast_node_list(iotas.0.gen_next_func.clone()),
                // maps: iotas.0.maps,
                maps: vector![],
            }));

        Ok(state)
    }
}
