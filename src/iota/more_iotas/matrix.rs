use std::{ops::Not, rc::Rc};

use downcast_rs::{impl_downcast, Downcast};
use nalgebra::{DMatrix, Dyn, Matrix, dmatrix};

use crate::{
    interpreter::state::Either3,
    iota::{
        hex_casting::{number::NumberIota, vector::VectorIota},
        Iota,
    },
};

pub type MatrixIota = Matrix<NumberIota, Dyn, Dyn, nalgebra::VecStorage<NumberIota, Dyn, Dyn>>;

impl Iota for MatrixIota {
    fn display(&self) -> String {
        let mut out = vec![];
        for row in self.row_iter() {
            let row_out = row.iter().map(f32::to_string).collect::<Vec<_>>();
            let row_str = format!("{}", row_out.join(", "));
            out.push(row_str)
        }
        format!(
            "[({}, {}) | {}]",
            self.row_iter().len(),
            self.column_iter().len(),
            out.join("; ")
        )
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<MatrixIota>() {
            Some(other) => {
                self.column(0).len() == other.column(0).len()
                    && self.row(0).len() == other.row(0).len()
                    && self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .map(|(rhs, lhs)| Iota::tolerates_other(rhs, lhs))
                        .collect::<im::Vector<bool>>()
                        .contains(&false)
                        .not()
            }
            None => false,
        }
    }
}

pub trait AsMatrix {
    fn as_matrix(&self) -> MatrixIota;
}

impl<T: AsMatrix, U: AsMatrix, V: AsMatrix> AsMatrix for Either3<Rc<T>, Rc<U>, Rc<V>> {
    fn as_matrix(&self) -> MatrixIota {
        match self {
            Either3::L(l) => l.as_matrix(),
            Either3::M(m) => m.as_matrix(),
            Either3::R(r) => r.as_matrix(),
        }
    }
}


impl AsMatrix for NumberIota {
    fn as_matrix(&self) -> MatrixIota {
        dmatrix![*self]
    }
}

impl AsMatrix for VectorIota {
    fn as_matrix(&self) -> MatrixIota {
        DMatrix::from_vec(3, 1, self.data.as_slice().to_vec())
    }
}

impl AsMatrix for MatrixIota {
    fn as_matrix(&self) -> MatrixIota {
        self.clone()
    }
}