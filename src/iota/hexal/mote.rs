use crate::iota::{hex_casting::null::NullIota, more_iotas::string, Iota};

#[derive(Debug)]
pub struct MoteIota {
    pub name: String,
    pub count: i32,
    pub storage: String,
    pub index: String,
}

impl Iota for MoteIota {
    fn display(&self) -> String {
        self.name.clone()
    }

    //idk if this is right or not
    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<MoteIota>() {
            Some(other) => {
                (self.storage == other.storage && self.index == other.index)
                    || (self.count == 0 && other.count == 0)
            }
            None => self.count == 0 && (other.downcast_ref::<NullIota>().is_some()),
        }
    }

    fn serialize_to_nbt(&self) -> String {
        todo!()
    }

    fn display_type_name() -> String
    where
        Self: Sized,
    {
        todo!()
    }
}
