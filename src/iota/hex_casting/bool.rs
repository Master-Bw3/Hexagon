use crate::iota::Iota;


pub type BooleanIota = bool;

impl Iota for BooleanIota {
    fn display(&self) -> String {
        self.to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<BooleanIota>() {
            Some(other) => other == self,
            None => false,
        }
    }
}