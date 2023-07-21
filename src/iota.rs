use std::fmt::Debug;

use downcast_rs::{impl_downcast, Downcast};

pub mod hex_casting;
pub mod more_iotas;

pub trait Iota: Downcast + Debug {
    fn display(&self) -> String;
    fn tolerates_other(&self, other: &dyn Iota) -> bool;
}
impl_downcast!(Iota);

// impl Iota for Rc<dyn Iota> {
//     fn display(&self) -> String {
//         self.display()
//     }

//     fn tolerates_other(&self, other: &dyn Iota) -> bool {
//         self.tolerates_other(other)
//     }
// }
