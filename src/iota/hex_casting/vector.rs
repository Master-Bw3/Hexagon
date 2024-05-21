use serde_json::{Map, Number};

use super::number::NumberIota;
use crate::iota::Iota;

pub type VectorIota = nalgebra::Matrix1x3<NumberIota>;

impl Iota for VectorIota {
    fn display(&self) -> String {
        format!("({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }

    fn display_type_name() -> String {
        "Vector".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        let tolerance = 0.0001;
        match other.downcast_ref::<VectorIota>() {
            Some(other) => (self.norm() - other.norm()).abs() < tolerance,
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        let x = self.x.to_bits() as i64;
        let y = self.y.to_bits() as i64;
        let z = self.z.to_bits() as i64;

        format!("{{\"hexcasting:type\": \"hexcasting:vec3\", \"hexcasting:data\": [L; {x}L, {y}L, {z}L]}}")
    }
    
    fn serialize_to_json(&self) -> serde_json::Value {
        let mut vec_map = Map::new();
        vec_map.insert("x".to_string(), serde_json::Value::Number(Number::from_f64(self.x).unwrap()));
        vec_map.insert("y".to_string(), serde_json::Value::Number(Number::from_f64(self.y).unwrap()));
        vec_map.insert("z".to_string(), serde_json::Value::Number(Number::from_f64(self.z).unwrap()));

        let mut map = Map::new();
        map.insert("iota_type".to_string(), serde_json::Value::String("vector".to_string()));
        map.insert("value".to_string(), serde_json::Value::Object(vec_map));

        serde_json::Value::Object(map)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::matrix;

    use super::*;

    #[test]
    fn test() {
        let vec: VectorIota = matrix![1.0, 0.25, 2.0];
        println!("{}", vec.serialize_to_nbt())
    }
}
