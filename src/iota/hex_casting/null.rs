use crate::iota::Iota;

#[derive(Debug, Clone, PartialEq)]
pub struct NullIota;

impl Iota for NullIota {
    fn display(&self) -> String {
        "Null".to_string()
    }

    fn display_type_name() -> String {
        "Null".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        other.downcast_ref::<NullIota>().is_some()
    }

    fn serialize_to_nbt(&self) -> String {
        "{\"hexcasting:type\": \"hexcasting:null\", \"hexcasting:data\": {}}".to_string()
    }
}
