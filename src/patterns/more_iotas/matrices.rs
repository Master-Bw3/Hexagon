use std::rc::Rc;

use im::{vector, Vector};
use nalgebra::{
    dmatrix, DMatrix, Matrix1xX,
};

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{Iota, more_iotas::matrix::MatrixIota, hex_casting::{number::NumberIota, vector::VectorIota, list::ListIota}},
    pattern_registry::PatternRegistry,
};

pub fn make<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota_a_b_or_c::<NumberIota, VectorIota, ListIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    fn map_num(element: &Rc<dyn Iota>) -> Result<f32, ()> {
        element.downcast_ref::<NumberIota>().ok_or(()).cloned()
    }

    fn matrix_from_vec_list(list: &Vector<Rc<dyn Iota>>) -> Result<MatrixIota, ()> {
        let row_list = list
            .iter()
            .map(|element| match element.clone().downcast_rc::<VectorIota>() {
                Ok(vec) => Ok(Matrix1xX::from_vec(vec![vec.x, vec.y, vec.z])),
                Err(_) => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DMatrix::from_rows(&row_list[..]))
    }

    fn matrix_from_num_list(list: &Vector<Rc<dyn Iota>>) -> Result<MatrixIota, ()> {
        let row = row_from_num_list(list)?;

        Ok(DMatrix::from_rows(&[row]))
    }

    fn row_from_num_list(list: &Vector<Rc<dyn Iota>>) -> Result<Matrix1xX<NumberIota>, ()> {
        let num_list = list.iter().map(map_num).collect::<Result<Vec<_>, _>>()?;

        Ok(Matrix1xX::from_vec(num_list))
    }

    fn matrix_from_num_list_list(list: &Vector<Rc<dyn Iota>>) -> Result<MatrixIota, ()> {
        let empty_vec: Rc<dyn Iota> = Rc::new(vector![]);
        let first_row = list.get(0).unwrap_or(&empty_vec);

        //used to ensure all rows have same length
        let row_len = match first_row.clone().downcast_rc::<ListIota>() {
            Ok(x) => Ok(x.len()),
            Err(_) => Err(()),
        }?;

        let num_num_list = list
            .iter()
            .map(|row| match row.clone().downcast_rc::<ListIota>() {
                Ok(inner_list) => {
                    if inner_list.len() == row_len {
                        row_from_num_list(&inner_list)
                    } else {
                        Err(())
                    }
                }
                Err(_) => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DMatrix::from_rows(&num_num_list[..]))
    }

    let operation_result = match iota {
        crate::interpreter::state::Either3::L(num) => dmatrix![*num],
        crate::interpreter::state::Either3::M(vec) => dmatrix![vec.x; vec.y; vec.z;],
        crate::interpreter::state::Either3::R(list) => matrix_from_num_list(&list)
            .or_else(|_| matrix_from_num_list_list(&list))
            .or_else(|_| matrix_from_vec_list(&list))
            .map_err(|_| {
                Mishap::IncorrectIota(1, "Number, Vector, or List".to_string(), list)
            })?,
    };

    state.stack.push_back(Rc::new(operation_result));

    Ok(state)
}
