use crate::pattern_registry::{PatternRegistry, PatternRegistryExt};

#[derive(Debug, Clone, PartialEq)]
pub enum Iota {
    Number(NumberIota),
    Vector(VectorIota),
    Pattern(PatternIota),
    Bool(BoolIota),
    Garbage(GarbageIota),
    Null(NullIota),
    Entity(EntityIota),
    List(ListIota),
}

pub type NumberIota = f32;
pub type BoolIota = bool;
pub type ListIota = std::vec::Vec<Iota>;
pub type VectorIota = nalgebra::Matrix1x3<NumberIota>;

#[derive(Debug, Clone, PartialEq)]
pub enum GarbageIota {
    Garbage,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NullIota {
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityIota {
    pub name: String,
    pub entity_type: String,
}

pub type PatternIota = Vec<PatternSigDir>;

#[derive(Debug, Clone, PartialEq)]
pub enum PatternSigDir {
    Q,
    A,
    S,
    D,
    E,
    W,
}

pub trait PatternIotaExt {
    fn from_sig(string: &str) -> PatternIota;
    fn from_name(registry: &PatternRegistry, string: &str) -> PatternIota;
}

impl PatternIotaExt for PatternIota {
    fn from_sig(string: &str) -> PatternIota {
        string
            .chars()
            .map(|char| match char {
                'q' => PatternSigDir::Q,
                'a' => PatternSigDir::A,
                's' => PatternSigDir::S,
                'd' => PatternSigDir::D,
                'e' => PatternSigDir::E,
                'w' => PatternSigDir::W,
                _ => panic!("invalid signature"),
            })
            .collect()
    }

    fn from_name(registry: &PatternRegistry, string: &str) -> PatternIota {
        PatternIota::from_sig(&registry.find(string.to_string()).unwrap().signature)
    }
}
