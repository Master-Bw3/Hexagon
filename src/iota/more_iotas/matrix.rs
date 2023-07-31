use std::{fmt::format, ops::Not, rc::Rc};

use nalgebra::{dmatrix, iter::RowIter, DMatrix, Dyn, Matrix};

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
            let row_out = row.iter().map(f64::to_string).collect::<Vec<_>>();
            let row_str = row_out.join(", ");
            out.push(row_str)
        }

        let formatted_out = if self.ncols() == 0 || self.nrows() == 0 {
            "".to_string()
        } else {
            format!(" | {}", out.join("; "))
        };
        format!("[({}, {}){}]", self.nrows(), self.ncols(), formatted_out)
    }

    fn display_type_name() -> String {
        "Matrix".to_string()
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

    fn serialize_to_nbt(&self) -> String {
        let gen_col_str = |col: Col<'_>| {
            let row_out = col
                .iter()
                .map(|x| format!("{x}d"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{row_out}]")
        };
        let out = self
            .column_iter()
            .map(gen_col_str)
            .collect::<Vec<_>>()
            .join(", ");
        let out = format!("[{out}]");

        format!("{{\"hexcasting:type\": \"moreiotas:matrix\", \"hexcasting:data\": {{mat: {out}, cols: {}, rows: {}}}}}", self.ncols(), self.nrows())
    }
}

type Col<'a> = Matrix<
    f64,
    Dyn,
    nalgebra::Const<1>,
    nalgebra::ViewStorage<'a, f64, Dyn, nalgebra::Const<1>, nalgebra::Const<1>, Dyn>,
>;

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
