use std::fmt::Debug;

use downcast_rs::{impl_downcast, Downcast};

pub mod hex_casting;
pub mod more_iotas;

pub trait Iota: Downcast + Debug {
    fn display(&self) -> String;
    fn tolerates_other(&self, other: &dyn Iota) -> bool;
}
impl_downcast!(Iota);

// impl dyn Iota {
//     // pub fn check_equality(&self, other: &Iota) -> bool {
//     //     let tolerance = 0.001;

//     //     match (self, other) {
//     //         (Iota::Number(a), Iota::Number(b)) => (a - b).abs() < tolerance,
//     //         (Iota::Vector(a), Iota::Vector(b)) => (a.norm() - b.norm()).abs() < tolerance,
//     //         (Iota::Pattern(a), Iota::Pattern(b)) => a.signature == b.signature,
//     //         (Iota::Bool(a), Iota::Bool(b)) => a == b,
//     //         (Iota::Garbage(_), Iota::Garbage(_)) => true,
//     //         (Iota::Null(_), Iota::Null(_)) => true,
//     //         (Iota::Entity(a), Iota::Entity(b)) => a == b,
//     //         (Iota::List(a), Iota::List(b)) => (a
//     //             .iter()
//     //             .zip(b.iter())
//     //             .map(|(a, b)| Iota::check_equality(a, b)))
//     //         .collect::<Vec<bool>>()
//     //         .contains(&false)
//     //         .not(),
//     //         _ => false,
//     //     }
//     // }

    

//     pub fn display_type(&self) -> &'static str {
//         match self {
//             Iota::Number(_) => "Number",
//             Iota::Vector(_) => "Vector",
//             Iota::Pattern(_) => "Pattern",
//             Iota::Bool(_) => "Bool",
//             Iota::Garbage(_) => "Garbage",
//             Iota::Null(_) => "Null",
//             Iota::Entity(_) => "Entity",
//             Iota::List(_) => "List",
//             Iota::Matrix(_) => "Matrix",
//             Iota::Continuation(_) => "Continuation",
//         }
//     }

//     pub fn display(&self) -> String {
//         match self {
//             Iota::Number(num) => num.display(),
//             Iota::Vector(vec) => vec.display(),
//             Iota::Pattern(pat) => pat.display(),
//             Iota::Bool(bool) => bool.display(),
//             Iota::Garbage(garbage) => garbage.display(),
//             Iota::Null(null) => null.display(),
//             Iota::Entity(name) => name.display(),
//             Iota::List(list) => list.display(),
//             Iota::Matrix(matrix) => matrix.display(),
//             Iota::Continuation(continuation) => continuation.display(),
//         }
//     }
// }

// impl std::cmp::PartialEq for dyn Iota {
//     fn eq(&self, other: &Self) -> bool {
//         self.check_equality(other)
//     }
// }

// impl std::fmt::Debug for Iota {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Number(arg0) => f.debug_tuple("Number").field(&arg0.display()).finish(),
//             Self::Vector(arg0) => f.debug_tuple("Vector").field(&arg0.display()).finish(),
//             Self::Pattern(arg0) => f.debug_tuple("Pattern").field(&arg0.display()).finish(),
//             Self::Bool(arg0) => f.debug_tuple("Bool").field(&arg0.display()).finish(),
//             Self::Garbage(arg0) => f.debug_tuple("Garbage").field(&arg0.display()).finish(),
//             Self::Null(arg0) => f.debug_tuple("Null").field(&arg0.display()).finish(),
//             Self::Entity(arg0) => f.debug_tuple("Entity").field(&arg0.display()).finish(),
//             Self::List(arg0) => f.debug_tuple("List").field(&arg0.display()).finish(),
//             Self::Continuation(arg0) => f.debug_tuple("Continuation").field(&arg0.display()).finish(),
//             Self::Matrix(arg0) => f.debug_tuple("Matrix").field(&arg0.display()).finish(),
//         }
//     }
// }



// pub type ListIota = std::vec::Vec<Iota>;

// impl Display for ListIota {
//     fn display(&self) -> String {
//         format!(
//             "[{}]",
//             self.iter()
//                 .map(Iota::display)
//                 .collect::<Vec<String>>()
//                 .join(", ")
//         )
//     }
// }

// pub type VectorIota = nalgebra::Matrix1x3<NumberIota>;

// impl Display for VectorIota {
//     fn display(&self) -> String {
//         format!("({}, {}, {})", self.x, self.y, self.z)
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum GarbageIota {
//     Garbage,
// }

// impl Display for GarbageIota {
//     fn display(&self) -> String {
//         "Garbage".to_string()
//     }
// }



// //reference to an entity


// #[derive(Debug, Clone, PartialEq)]
// pub struct PatternIota {
//     pub signature: Signature,
//     pub value: Box<Option<ActionValue>>,
// }



// impl Display for PatternIota {
//     fn display(&self) -> String {
//         let mut result = PatternRegistry::find(
//             //todo: maybe make this not generate the entire registry each time
//             &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
//             &self.signature.as_str(),
//             &self.value,
//         )
//         .map_or(self.signature.as_str(), |pat| pat.display_name);

//         if let Some(value) = *self.value.clone() {
//             match value {
//                 ActionValue::Iota(iota) => result = format!("{result}: {}", iota.display()),
//                 ActionValue::Bookkeeper(code) => result = format!("{result}: {code}"),
//             }
//         }
//         result
//     }
// }

// pub type Signature = Vec<PatternSigDir>;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum PatternSigDir {
//     Q,
//     A,
//     S,
//     D,
//     E,
//     W,
// }

// pub trait SignatureExt {
//     fn from_sig(string: &str) -> Signature;
//     fn from_name(
//         registry: &PatternRegistry,
//         string: &str,
//         value: &Option<ActionValue>,
//     ) -> Option<Signature>;
//     fn as_str(&self) -> String;
// }

// impl SignatureExt for Signature {
//     fn from_sig(string: &str) -> Signature {
//         string
//             .chars()
//             .map(|char| match char {
//                 'q' => PatternSigDir::Q,
//                 'a' => PatternSigDir::A,
//                 's' => PatternSigDir::S,
//                 'd' => PatternSigDir::D,
//                 'e' => PatternSigDir::E,
//                 'w' => PatternSigDir::W,
//                 _ => panic!("invalid signature: {}", string),
//             })
//             .collect()
//     }

//     fn from_name(
//         registry: &PatternRegistry,
//         string: &str,
//         value: &Option<ActionValue>,
//     ) -> Option<Signature> {
//         Some(Signature::from_sig(
//             &registry.find(string, value)?.signature,
//         ))
//     }

//     fn as_str(&self) -> String {
//         self.iter()
//             .map(|char| match char {
//                 PatternSigDir::Q => 'q',
//                 PatternSigDir::A => 'a',
//                 PatternSigDir::S => 's',
//                 PatternSigDir::D => 'd',
//                 PatternSigDir::E => 'e',
//                 PatternSigDir::W => 'w',
//             })
//             .collect()
//     }
// }


