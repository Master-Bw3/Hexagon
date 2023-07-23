use std::rc::Rc;

use im::{vector, Vector};
use nalgebra::{dmatrix, DMatrix, Matrix1xX, MatrixXx1};

use crate::{
    interpreter::{
        mishap::{MatrixSize, Mishap},
        state::{Either3, StackExt, State},
    },
    iota::{
        hex_casting::{
            list::ListIota,
            number::{NumberIota, NumberIotaExt},
            vector::VectorIota,
        },
        more_iotas::matrix::{AsMatrix, MatrixIota},
        Iota,
    },
    pattern_registry::PatternRegistry,
};

pub fn make<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state
        .stack
        .get_iota_a_b_or_c::<NumberIota, VectorIota, ListIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    fn map_num(element: &Rc<dyn Iota>) -> Result<f32, ()> {
        element.downcast_ref::<NumberIota>().ok_or(()).cloned()
    }

    fn matrix_from_vec_list(list: &Vector<Rc<dyn Iota>>) -> Result<MatrixIota, ()> {
        let row_list = list
            .iter()
            .map(
                |element| match element.clone().downcast_rc::<VectorIota>() {
                    Ok(vec) => Ok(MatrixXx1::from_vec(vec![vec.x, vec.y, vec.z])),
                    Err(matrix) => Err(()),
                },
            )
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DMatrix::from_columns(&row_list[..]))
    }

    fn matrix_from_num_list(list: &Vector<Rc<dyn Iota>>) -> Result<MatrixIota, ()> {
        let row = row_from_num_list(list)?;

        Ok(DMatrix::from_rows(&[row]))
    }

    fn col_from_num_list(list: &Vector<Rc<dyn Iota>>) -> Result<MatrixXx1<NumberIota>, ()> {
        let num_list = list.iter().map(map_num).collect::<Result<Vec<_>, _>>()?;

        Ok(MatrixXx1::from_vec(num_list))
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
            Err(matrix) => Err(()),
        }?;

        let num_num_list = list
            .iter()
            .map(|row| match row.clone().downcast_rc::<ListIota>() {
                Ok(inner_list) => {
                    if inner_list.len() == row_len {
                        col_from_num_list(&inner_list)
                    } else {
                        Err(())
                    }
                }
                Err(matrix) => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DMatrix::from_columns(&num_num_list[..]))
    }

    let operation_result = match iota {
        Either3::L(num) => dmatrix![*num],
        Either3::M(vec) => dmatrix![vec.x; vec.y; vec.z;],
        Either3::R(list) => matrix_from_num_list(&list)
            .or_else(|_| matrix_from_num_list_list(&list))
            .or_else(|_| matrix_from_vec_list(&list))
            .map_err(|_| Mishap::IncorrectIota(1, "Number, Vector, or List".to_string(), list))?,
    };

    state.stack.push_back(Rc::new(operation_result));

    Ok(state)
}

pub fn unmake<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let matrix = state.stack.get_iota::<MatrixIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = if matrix.len() == 1 {
        Rc::new(matrix[0])
    } else if (matrix.column_iter().len() == 1 && matrix.row_iter().len() == 3)
        || (matrix.column_iter().len() == 3 && matrix.row_iter().len() == 1)
    {
        Rc::new(VectorIota::new(matrix[0], matrix[1], matrix[2]))
    } else {
        let mut list: Vector<Rc<dyn Iota>> = vector![];
        if matrix.row_iter().len() == 3 {
            for row in matrix.column_iter() {
                list.push_back(Rc::new(VectorIota::new(row[0], row[1], row[2])))
            }
        } else if matrix.column_iter().len() == 3 {
            for col in matrix.row_iter() {
                list.push_back(Rc::new(VectorIota::new(col[0], col[1], col[2])))
            }
        } else {
            for (c, _) in matrix.column_iter().enumerate() {
                let mut to_add: Vector<Rc<dyn Iota>> = vector![];
                for (r, _) in matrix.row_iter().enumerate() {
                    to_add.push_back(Rc::new(matrix.row(r)[c]))
                }

                list.push_back(Rc::new(to_add))
            }
        }

        Rc::new(list)
    };

    state.stack.push_back(operation_result);
    Ok(state)
}

pub fn identity<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state
        .stack
        .get_iota::<NumberIota>(0, arg_count)?
        .positive_int(0)? as usize;
    state.stack.remove_args(&arg_count);

    let identity_matrix: Rc<dyn Iota> = Rc::new(MatrixIota::identity(iota, iota));

    state.stack.push_back(identity_matrix);

    Ok(state)
}

pub fn zero<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let n = state
        .stack
        .get_iota::<NumberIota>(0, arg_count)?
        .positive_int(0)? as usize;
    let m = state
        .stack
        .get_iota::<NumberIota>(1, arg_count)?
        .positive_int(0)? as usize;
    state.stack.remove_args(&arg_count);

    let identity_matrix: Rc<dyn Iota> = Rc::new(MatrixIota::zeros(n, m));

    state.stack.push_back(identity_matrix);

    Ok(state)
}

pub fn rotate<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let axis = state.stack.get_iota::<VectorIota>(0, arg_count)?;
    let theta = state.stack.get_iota::<NumberIota>(1, arg_count)?;
    state.stack.remove_args(&arg_count);

    let (x, y, z) = (axis.x, axis.y, axis.z);
    let c = theta.cos();
    let s = theta.sin();
    let nc = 1.0 - c;

    let matrix: MatrixIota = dmatrix![c + x*x*nc, x*y*nc - z*s, x*z*nc + y*s; 
                                      y*x*nc + z*s, c + y*y*nc, y*z*nc - x*s; 
                                      z*x*nc - y*s, z*y*nc + x*s, c + z*z*nc];

    let identity_matrix: Rc<dyn Iota> = Rc::new(matrix);

    state.stack.push_back(identity_matrix);

    Ok(state)
}

pub fn add<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let lhs = state
        .stack
        .get_iota_a_b_or_c::<NumberIota, VectorIota, MatrixIota>(0, arg_count)?
        .as_matrix();
    let rhs = state
        .stack
        .get_iota_a_b_or_c::<NumberIota, VectorIota, MatrixIota>(1, arg_count)?
        .as_matrix();
    state.stack.remove_args(&arg_count);

    if lhs.row_iter().len() == rhs.row_iter().len()
        && lhs.column_iter().len() == rhs.row_iter().len()
    {
        state.stack.push_back(Rc::new(lhs + rhs));
        Ok(state)
    } else {
        Err(Mishap::MatrixWrongSize(
            Rc::new(rhs.clone()),
            MatrixSize::Const(lhs.row_iter().len()),
            MatrixSize::Const(rhs.row_iter().len()),
        ))
    }
}

pub fn multiply<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let lhs = state
        .stack
        .get_iota_a_b_or_c::<NumberIota, VectorIota, MatrixIota>(0, arg_count)?;
    let rhs = state
        .stack
        .get_iota_a_b_or_c::<NumberIota, VectorIota, MatrixIota>(1, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match (lhs, rhs) {
        (Either3::L(num1), Either3::L(num2)) => Rc::new(*num1 * *num2),
        (Either3::L(num), Either3::M(vec)) | (Either3::M(vec), Either3::L(num)) => {
            Rc::new(*vec * *num)
        }
        (Either3::L(num), Either3::R(matrix)) | (Either3::R(matrix), Either3::L(num)) => {
            Rc::new((*matrix).clone() * *num)
        }
        (Either3::M(_vec1), Either3::M(vec2)) => Err(Mishap::MatrixWrongSize(
            vec2,
            MatrixSize::Const(1),
            MatrixSize::N,
        ))?,
        //if both are vectors/matrices
        (lhs, rhs) => {
            let matrix1 = lhs.as_matrix();
            let matrix2 = rhs.as_matrix();
            if matrix1.column_iter().len() == matrix2.row_iter().len() {
                Rc::new(matrix1 * matrix2)
            } else {
                Err(Mishap::MatrixWrongSize(
                    Rc::new(matrix2),
                    MatrixSize::Const(1),
                    MatrixSize::N,
                ))?
            }

        }
    };

    state.stack.push_back(operation_result);

    Ok(state)
}
