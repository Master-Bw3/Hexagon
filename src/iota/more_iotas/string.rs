use crate::iota::Iota;

pub type StringIota = String;

impl Iota for StringIota {
    fn display(&self) -> String {
        format!("\"{}\"", self)
    }
    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<StringIota>() {
            Some(other) => self == other,
            None => false,
        }
    }
}
