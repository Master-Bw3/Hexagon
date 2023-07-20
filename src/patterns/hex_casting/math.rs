use im::Vector;
use std::ops::Deref;
use std::{f32::consts::PI, ops::Not, rc::Rc};

use crate::iota::hex_casting::number::NumberIotaExt;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::{
        hex_casting::{
            bool::BooleanIota, list::ListIota, null::NullIota, number::NumberIota,
            vector::VectorIota,
        },
        Iota,
    },
    pattern_registry::PatternRegistry,
};

pub fn add<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(0, arg_count)?,
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iotas {
        (Either::L(num1), Either::L(num2)) => Rc::new(*num1 + *num2),
        (Either::L(num), Either::R(vec)) => Rc::new(vec.add_scalar(*num)),
        (Either::R(vec), Either::L(num)) => Rc::new(vec.add_scalar(*num)),
        (Either::R(vec1), Either::R(vec2)) => Rc::new(*vec1 + *vec2),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn subtract<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(0, arg_count)?,
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iotas {
        (Either::L(num1), Either::L(num2)) => Rc::new(*num1 - *num2),
        (Either::L(num), Either::R(vec)) => Rc::new((*vec * -1.0).add_scalar(*num)),
        (Either::R(vec), Either::L(num)) => Rc::new(vec.add_scalar(*num * -1.0)),
        (Either::R(vec1), Either::R(vec2)) => Rc::new(*vec1 - *vec2),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn mul_dot<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(0, arg_count)?,
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iotas {
        (Either::L(num1), Either::L(num2)) => Rc::new(*num1 * *num2),
        (Either::L(num), Either::R(vec)) => Rc::new(*vec * *num),
        (Either::R(vec), Either::L(num)) => Rc::new(*vec * *num),
        (Either::R(vec1), Either::R(vec2)) => Rc::new(vec1.dot(&vec2)),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn div_cross<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(0, arg_count)?,
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iotas {
        (Either::L(num1), Either::L(num2)) => Rc::new(*num1 / *num2),
        (Either::L(num), Either::R(vec)) => Rc::new(*vec / *num),
        (Either::R(vec), Either::L(num)) => Rc::new(*vec / *num),
        (Either::R(vec1), Either::R(vec2)) => Rc::new(vec1.cross(&vec2)),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn abs_len<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state
        .stack
        .get_iota_a_or_b::<NumberIota, VectorIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iota {
        Either::L(num) => Rc::new(num.abs()),
        Either::R(vec) => Rc::new(vec.norm()),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn pow_proj<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(0, arg_count)?,
        state
            .stack
            .get_iota_a_or_b::<NumberIota, VectorIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iotas {
        (Either::L(num1), Either::L(num2)) => Rc::new(num1.powf(*num2)),
        (Either::L(num), Either::R(vec)) => Rc::new(VectorIota::new(
            num.powf(vec.x),
            num.powf(vec.y),
            num.powf(vec.z),
        )),
        (Either::R(vec), Either::L(num)) => Rc::new(VectorIota::new(
            vec.x.powf(*num),
            vec.y.powf(*num),
            vec.z.powf(*num),
        )),
        (Either::R(vec1), Either::R(vec2)) => {
            let projection_piece = |num: f32| num * vec2.dot(&vec1) / vec1.dot(&vec2);
            Rc::new(vec1.map(projection_piece))
        }
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn floor<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state
        .stack
        .get_iota_a_or_b::<NumberIota, VectorIota>(0, 1)?;
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iota {
        Either::L(num) => Rc::new(num.floor()),
        Either::R(vec) => Rc::new(vec.map(f32::floor)),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn ceil<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state
        .stack
        .get_iota_a_or_b::<NumberIota, VectorIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iota {
        Either::L(num) => Rc::new(num.ceil()),
        Either::R(vec) => Rc::new(vec.map(f32::ceil)),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn construct_vec<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let iotas = (
        *state.stack.get_iota::<NumberIota>(0, arg_count)?,
        *state.stack.get_iota::<NumberIota>(1, arg_count)?,
        *state.stack.get_iota::<NumberIota>(2, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    state
        .stack
        .push(Rc::new(VectorIota::new(iotas.0, iotas.1, iotas.2)));

    Ok(state)
}

pub fn deconstruct_vec<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<VectorIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    state.stack.push(Rc::new(iota.x));
    state.stack.push(Rc::new(iota.y));
    state.stack.push(Rc::new(iota.z));

    Ok(state)
}

pub fn coerce_axial<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<VectorIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = {
        let magnitude = iota.norm();
        let azimuth = f32::acos(iota.z / magnitude);
        let theta: f32 = f32::atan2(iota.y, iota.x);
        let snapped_azimuth = (PI / 2.0) * (azimuth / (PI / 2.0)).round();
        let snapped_theta = (PI / 2.0) * (theta / (PI / 2.0)).round();

        VectorIota::new(
            snapped_azimuth.sin() * snapped_theta.cos(),
            snapped_azimuth.sin() * snapped_theta.sin(),
            snapped_azimuth.cos(),
        )
    };

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn and<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<BooleanIota>(0, arg_count)?,
        *state.stack.get_iota::<BooleanIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = iotas.0 & iotas.1;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn or<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<BooleanIota>(0, arg_count)?,
        *state.stack.get_iota::<BooleanIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = iotas.0 | iotas.1;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn xor<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<BooleanIota>(0, arg_count)?,
        *state.stack.get_iota::<BooleanIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = iotas.0 ^ iotas.1;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn greater<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<NumberIota>(0, arg_count)?,
        *state.stack.get_iota::<NumberIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = iotas.0 > iotas.1;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn less<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<NumberIota>(0, arg_count)?,
        *state.stack.get_iota::<NumberIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = iotas.0 < iotas.1;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn greater_eq<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<NumberIota>(0, arg_count)?,
        *state.stack.get_iota::<NumberIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = iotas.0 >= iotas.1;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn less_eq<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<NumberIota>(0, arg_count)?,
        *state.stack.get_iota::<NumberIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = iotas.0 <= iotas.1;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn equals<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    let operation_result = (*iotas.0).tolerates_other(&*iotas.1);

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn not_equals<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_any_iota(0, arg_count)?.clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    let operation_result = !((*iotas.0).tolerates_other(&*iotas.1));

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn not<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = *state.stack.get_iota::<BooleanIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = !iota;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn bool_coerce<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_any_iota(0, arg_count)?.clone();
    state.stack.remove_args(&arg_count);

    let operation_result = if let Ok(x) = iota.clone().downcast_rc::<NumberIota>() {
        (*x).tolerates_other(&0.0)
    } else if let Ok(x) = iota.clone().downcast_rc::<BooleanIota>() {
        *x
    } else if let Ok(x) = iota.clone().downcast_rc::<ListIota>() {
        !x.is_empty()
    } else if iota.clone().downcast_rc::<NullIota>().is_ok() {
        //redundant I think
        false
    } else {
        false
    };

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn sin<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<NumberIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = iota.sin();

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn cos<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<NumberIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = iota.cos();

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn tan<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<NumberIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = iota.tan();

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn arcsin<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<NumberIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = iota.asin();

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn arccos<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<NumberIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = iota.acos();

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn arctan<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<NumberIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = iota.atan();

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn logarithm<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<NumberIota>(0, arg_count)?,
        *state.stack.get_iota::<NumberIota>(1, arg_count)?,
    );

    state.stack.remove_args(&arg_count);

    let operation_result = (iotas.0).log(iotas.1);

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn modulo<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        *state.stack.get_iota::<NumberIota>(0, arg_count)?,
        *state.stack.get_iota::<NumberIota>(1, arg_count)?,
    );

    state.stack.remove_args(&arg_count);

    let operation_result = iotas.0 % iotas.1;

    state.stack.push(Rc::new(operation_result));

    Ok(state)
}

pub fn and_bit<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state
            .stack
            .get_iota_a_or_b::<NumberIota, ListIota>(0, arg_count)?,
        state
            .stack
            .get_iota_a_or_b::<NumberIota, ListIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iotas {
        (Either::L(num1), Either::L(num2)) => Rc::new(((*num1).int(0)? & (*num2).int(1)?) as f32),
        (Either::R(list1), Either::R(list2)) => Rc::new(
            list1
                .iter()
                .filter(|iota: &&Rc<dyn Iota>| {
                    list2
                        .iter()
                        .map(|i| i.tolerates_other((*iota).deref()))
                        .collect::<Vec<bool>>()
                        .contains(&true)
                })
                .cloned()
                .collect::<Vector<Rc<dyn Iota>>>(),
        ),
        (Either::L(_num), Either::R(list)) => {
            Err(Mishap::IncorrectIota(0, "Integer".to_string(), list))?
        }

        (Either::R(_list), Either::L(num)) => {
            Err(Mishap::IncorrectIota(0, "List".to_string(), num))?
        }
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn or_bit<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state
            .stack
            .get_iota_a_or_b::<NumberIota, ListIota>(0, arg_count)?,
        state
            .stack
            .get_iota_a_or_b::<NumberIota, ListIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iotas {
        (Either::L(num1), Either::L(num2)) => Rc::new(((*num1).int(0)? | (*num2).int(1)?) as f32),
        (Either::R(list1), Either::R(list2)) => {
            let mut new_list1 = list1.deref().clone();
            let mut new_list2 = list2.deref().clone();


            new_list2.retain(|iota| {
                list1
                    .iter()
                    .map(|i| i.tolerates_other(iota.deref()))
                    .collect::<Vec<bool>>()
                    .contains(&true)
                    .not()
            });

            new_list1.append(new_list2);
            list1
        }

        (Either::L(_num), Either::R(list)) => {
            Err(Mishap::IncorrectIota(0, "Integer".to_string(), list))?
        }

        (Either::R(_list), Either::L(num)) => {
            Err(Mishap::IncorrectIota(0, "List".to_string(), num))?
        }
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn xor_bit<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state
            .stack
            .get_iota_a_or_b::<NumberIota, ListIota>(0, arg_count)?,
        state
            .stack
            .get_iota_a_or_b::<NumberIota, ListIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match iotas {
        (Either::L(num1), Either::L(num2)) => Rc::new((num1.int(0)? ^ num2.int(1)?) as f32),
        (Either::R(list1), Either::R(list2)) => Rc::new({
            let mut new_list: Vector<_> = list1
                .iter()
                .filter(|iota| {
                    list2
                        .iter()
                        .map(|i| i.tolerates_other((*iota).deref()))
                        .collect::<Vec<bool>>()
                        .contains(&true)
                        .not()
                })
                .cloned()
                .collect();

            new_list.append(
                list2
                    .iter()
                    .filter(|iota| {
                        list1
                            .iter()
                            .map(|i| i.tolerates_other((*iota).deref()))
                            .collect::<Vec<bool>>()
                            .contains(&true)
                            .not()
                    })
                    .cloned()
                    .collect(),
            );
            new_list
        }),
        (Either::L(_num), Either::R(list)) => {
            Err(Mishap::IncorrectIota(0, "Integer".to_string(), list))?
        }

        (Either::R(_list), Either::L(num)) => {
            Err(Mishap::IncorrectIota(0, "List".to_string(), num))?
        }
    };

    state.stack.push(operation_result);

    Ok(state)
}

// pub fn not_bit<'a>(
//     state: &'a mut State,
//     _pattern_registry: &PatternRegistry,
// ) -> Result<&'a mut State, Mishap> {
//     let arg_count = 1;
//     let iota = state.stack.get_integer(0, arg_count)?;

//     state.stack.remove_args(&arg_count);

//     let operation_result: Rc<dyn Iota> = !iota;

//     state.stack.push((operation_result as f32));

//     Ok(state)
// }

// pub fn to_set<'a>(
//     state: &'a mut State,
//     _pattern_registry: &PatternRegistry,
// ) -> Result<&'a mut State, Mishap> {
//     let arg_count = 1;
//     let list = state.stack.get_list(0, arg_count)?;

//     state.stack.remove_args(&arg_count);

//     let operation_result: Rc<dyn Iota> = list.iter().fold(vec![], |acc, iota| {
//         if acc.contains(iota) {
//             acc
//         } else {
//             let mut new_acc = acc;
//             new_acc.push(iota.clone());
//             new_acc
//         }
//     });

//     state.stack.push(Iota::List(operation_result));

//     Ok(state)
// }

// pub fn bool_if<'a>(
//     state: &'a mut State,
//     _pattern_registry: &PatternRegistry,
// ) -> Result<&'a mut State, Mishap> {
//     let arg_count = 3;
//     let iotas = (
//         state.stack.get_iota::<BooleanIota>(0, arg_count)?,
//         state.stack.get_iota(1, arg_count)?.clone(),
//         state.stack.get_iota(2, arg_count)?.clone(),
//     );
//     state.stack.remove_args(&arg_count);

//     let operation_result: Rc<dyn Iota> = if iotas.0 { iotas.1 } else { iotas.2 };

//     state.stack.push(operation_result);

//     Ok(state)
// }

// pub fn random<'a>(
//     state: &'a mut State,
//     _pattern_registry: &PatternRegistry,
// ) -> Result<&'a mut State, Mishap> {
//     let rand = rand::random::<f32>();

//     state.stack.push((rand));

//     Ok(state)
// }

// #[cfg(test)]
// mod tests {

//     use crate::pattern_registry::PatternRegistryExt;

//     use super::*;

//     #[test]
//     fn and_bit_list_test() {
//         let mut state = State::default();
//         state.stack = vec![
//             Iota::List(vec![
//                 (1.0),
//                 (1.0),
//                 (2.0),
//             ]),
//             Iota::List(vec![(2.0), (3.0)]),
//         ];

//         let expected = vec![Iota::List(vec![(2.0)])];

//         let result = and_bit(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }

//     #[test]
//     fn or_bit_list_test() {
//         let mut state = State::default();
//         state.stack = vec![
//             Iota::List(vec![
//                 (1.0),
//                 (1.0),
//                 (2.0),
//                 (4.0),
//             ]),
//             Iota::List(vec![
//                 (1.0),
//                 (1.0),
//                 (2.0),
//                 (3.0),
//             ]),
//         ];

//         let expected = vec![Iota::List(vec![
//             (1.0),
//             (1.0),
//             (2.0),
//             (4.0),
//             (3.0),
//         ])];

//         let result = or_bit(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }

//     #[test]
//     fn xor_bit_list_test() {
//         let mut state = State::default();
//         state.stack = vec![
//             Iota::List(vec![
//                 (1.0),
//                 (1.0),
//                 (2.0),
//                 (4.0),
//             ]),
//             Iota::List(vec![
//                 (1.0),
//                 (2.0),
//                 (3.0),
//             ]),
//         ];

//         let expected = vec![Iota::List(vec![(4.0), (3.0)])];

//         let result = xor_bit(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }

//     #[test]
//     fn to_set_test() {
//         let mut state = State::default();
//         state.stack = vec![Iota::List(vec![
//             (1.0),
//             (1.0),
//             (2.0),
//             (3.0),
//             (1.0),
//         ])];

//         let expected = vec![Iota::List(vec![
//             (1.0),
//             (2.0),
//             (3.0),
//         ])];

//         let result = to_set(
//             &mut state,
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//         )
//         .unwrap();
//         assert_eq!(result.stack, expected)
//     }
// }
