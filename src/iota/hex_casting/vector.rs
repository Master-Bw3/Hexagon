use super::number::NumberIota;
use crate::iota::Iota;

pub type VectorIota = nalgebra::Matrix1x3<NumberIota>;

impl Iota for VectorIota {
    fn display(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        let tolerance = 0.001;
        match other.downcast_ref::<VectorIota>() {
            Some(other) => (self.norm() - other.norm()).abs() < tolerance,
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        let x = self.x.to_bits();
        let y = self.x.to_bits();
        let z = self.x.to_bits();

        format!("{{\"hexcasting:type\": \"hexcasting:vec3\", \"hexcasting:data\": [L; {x}L, {y}L, {z}L]}}")
    }
}

#[cfg(test)]
mod tests {

    use im::vector;
    use nalgebra::matrix;

    use crate::pattern_registry::PatternRegistryExt;

    use super::*;

    #[test]
    fn test() {
        let vec: VectorIota = matrix![1.0, 0.25, 2.0];
        println!("{}", vec.serialize_to_nbt())
    }
}
