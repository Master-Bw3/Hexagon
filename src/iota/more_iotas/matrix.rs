use std::ops::Not;

use nalgebra::{Dyn, Matrix};

use crate::iota::{hex_casting::number::NumberIota, Iota};

pub type MatrixIota = Matrix<NumberIota, Dyn, Dyn, nalgebra::VecStorage<NumberIota, Dyn, Dyn>>;

impl Iota for MatrixIota {
    fn display(&self) -> String {
        let mut out = vec![];
        for row in self.row_iter() {
            let row_out = row.iter().map(f32::to_string).collect::<Vec<_>>();
            let row_str = format!("{}", row_out.join(", "));
            out.push(row_str)
        }
        format!("({}, {}) | {}", self.row_iter().len(), self.column_iter().len(), out.join("; "))
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
