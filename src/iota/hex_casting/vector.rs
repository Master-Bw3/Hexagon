use super::number::NumberIota;
use crate::iota::Iota;

pub type VectorIota = nalgebra::Matrix1x3<NumberIota>;

impl Iota for VectorIota {
    fn display(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        let tolerance =  0.001;
        match other.downcast_ref::<VectorIota>() {
            Some(other) => (self.norm() - other.norm()).abs() < tolerance,
            None => false,
        }
    }
}