use crate::iota::Iota;

#[derive(Debug, Clone, PartialEq)]
pub enum GarbageIota {
    Garbage,
}

impl Iota for GarbageIota {
    fn display(&self) -> String {
        "Garbage".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        other.downcast_ref::<GarbageIota>().is_some()
    }
}