use crate::iota::Iota;

pub type NumberIota = f32;


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