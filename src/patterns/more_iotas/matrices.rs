use nalgebra::{
    coordinates::X, dmatrix, DMatrix, Dyn, Matrix, Matrix1x3, Matrix1xX, RowVector, VecStorage,
};

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::{Iota, ListIota, MatrixIota, NumberIota, VectorIota},
    pattern_registry::PatternRegistry,
};

pub fn make<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_num_or_vec_or_list(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    fn map_num(element: &Iota) -> Result<f32, ()> {
        match element {
            Iota::Number(num) => Ok(*num),
            _ => Err(()),
        }
    }

    fn matrix_from_vec_list(list: &Vec<Iota>) -> Result<MatrixIota, ()> {
        let row_list = list
            .iter()
            .map(|element| match element {
                Iota::Vector(vec) => Ok(Matrix1xX::from_vec(vec![vec.x, vec.y, vec.z])),
                _ => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DMatrix::from_rows(&row_list[..]))
    }

    fn matrix_from_num_list(list: &Vec<Iota>) -> Result<MatrixIota, ()> {
        let row = row_from_num_list(list)?;

        Ok(DMatrix::from_rows(&[row]))
    }

    fn row_from_num_list(list: &Vec<Iota>) -> Result<Matrix1xX<NumberIota>, ()> {
        let num_list = list.iter().map(map_num).collect::<Result<Vec<_>, _>>()?;

        Ok(Matrix1xX::from_vec(num_list))
    }

    fn matrix_from_num_list_list(list: &Vec<Iota>) -> Result<MatrixIota, ()> {
        let empty_vec = Iota::List(vec![]);
        let first_row = list.get(0).unwrap_or(&empty_vec);

        //used to ensure all rows have same length
        let row_len = match first_row {
            Iota::List(x) => Ok(x.len()),
            _ => Err(()),
        }?;

        let num_num_list = list
            .iter()
            .map(|row| match row {
                Iota::List(inner_list) => {
                    if inner_list.len() == row_len {
                        row_from_num_list(inner_list)
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DMatrix::from_rows(&num_num_list[..]))
    }

    let operation_result = match iota {
        crate::interpreter::state::Either3::L(num) => dmatrix![num],
        crate::interpreter::state::Either3::M(vec) => dmatrix![vec.x; vec.y; vec.z;],
        crate::interpreter::state::Either3::R(list) => matrix_from_num_list(&list)
            .or_else(|_| matrix_from_num_list_list(&list))
            .or_else(|_| matrix_from_vec_list(&list))
            .map_err(|_| {
                Mishap::IncorrectIota(1, "Number, Vector, or List".to_string(), Iota::List(list))
            })?,
    };

    state.stack.push(Iota::Matrix(operation_result));

    Ok(state)
}
