use std::rc::Rc;

use crate::{iota::Iota, interpreter::mishap::Mishap};

pub type NumberIota = f32;

pub trait NumberIotaExt {
    fn int(self, index: usize) -> Result<i32, Mishap>;
}

impl NumberIotaExt for NumberIota {
    fn int(self, index: usize) -> Result<i32, Mishap> {
        if self.round().tolerates_other(&self) {
            Ok(self as i32)
        } else {
            Err(Mishap::IncorrectIota(index, "Integer".to_string(), Rc::new(self)))
        }
    }
}

impl Iota for NumberIota {
    fn display(&self) -> String {
        self.to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        let tolerance =  0.001;
        match other.downcast_ref::<NumberIota>() {
            Some(other) => (self - other).abs() < tolerance,
            None => false,
        }
    }
}