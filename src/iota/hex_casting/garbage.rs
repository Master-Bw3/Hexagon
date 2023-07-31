use crate::iota::Iota;

#[derive(Debug, Clone, PartialEq)]
pub struct GarbageIota;

impl Iota for GarbageIota {
    fn display(&self) -> String {
        "Garbage".to_string()
    }

    fn display_type_name() -> String {
        "Garbage".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        other.downcast_ref::<GarbageIota>().is_some()
    }

    fn serialize_to_nbt(&self) -> String {
        "{\"hexcasting:type\": \"hexcasting:garbage\", \"hexcasting:data\": {}}".to_string()
    }
}
