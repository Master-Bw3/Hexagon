use crate::iota::Iota;

#[derive(Debug, Clone, PartialEq)]
pub enum NullIota {
    Null,
}

impl Iota for NullIota {
    fn display(&self) -> String {
        "Null".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        other.downcast_ref::<NullIota>().is_some()
    }
}