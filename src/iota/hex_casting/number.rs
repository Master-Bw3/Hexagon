use std::rc::Rc;

use serde_json::{Map, Number};

use crate::{interpreter::mishap::Mishap, iota::Iota};

pub type NumberIota = f64;

pub trait NumberIotaExt {
    fn int(self, index: usize) -> Result<i32, Mishap>;
    fn positive_int(self, index: usize) -> Result<i32, Mishap>;
    fn positive_int_under_inclusive(self, index: usize, len: usize) -> Result<i32, Mishap>;
}

impl NumberIotaExt for NumberIota {
    fn int(self, index: usize) -> Result<i32, Mishap> {
        let tolerance = 0.001;
        if (self - self.round()).abs() < tolerance {
            Ok(self as i32)
        } else {
            Err(Mishap::IncorrectIota {
                index,
                expected: "Integer".to_string(),
                received: Rc::new(self),
            })
        }
    }

    fn positive_int(self, index: usize) -> Result<i32, Mishap> {
        let int = self.int(index).map_err(|_| Mishap::IncorrectIota {
            index,
            expected: "Positive Integer".to_string(),
            received: Rc::new(self),
        })?;

        if int >= 0 {
            Ok(int)
        } else {
            Err(Mishap::IncorrectIota {
                index,
                expected: "Positive Integer".to_string(),
                received: Rc::new(self),
            })
        }
    }

    fn positive_int_under_inclusive(self, index: usize, len: usize) -> Result<i32, Mishap> {
        let int = self
            .positive_int(index)
            .map_err(|_| Mishap::IncorrectIota {
                index,
                expected: format!("Integer between 0 and {}", len),
                received: Rc::new(self),
            })?;
        if int <= len as i32 {
            Ok(int)
        } else {
            Err(Mishap::IncorrectIota {
                index,
                expected: format!("Integer between 0 and {}", len),
                received: Rc::new(self),
            })
        }
    }
}

impl Iota for NumberIota {
    fn display(&self) -> String {
        format!("{:.3}", self)
    }

    fn display_type_name() -> String {
        "Number".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        let tolerance = 0.0001;
        match other.downcast_ref::<NumberIota>() {
            Some(other) => (self - other).abs() < tolerance,
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        format!("{{\"hexcasting:type\": \"hexcasting:double\", \"hexcasting:data\": {self}d}}")
    }

    fn serialize_to_json(&self) -> serde_json::Value {
        let mut map = Map::new();
        map.insert(
            "iota_type".to_string(),
            serde_json::Value::String("number".to_string()),
        );
        map.insert(
            "value".to_string(),
            serde_json::Value::Number(Number::from_f64(*self).unwrap()),
        );

        serde_json::Value::Object(map)
    }
}
