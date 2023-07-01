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

#[derive(Debug, Clone, PartialEq)]
pub struct PatternIota {
    pub signature: Signature,
    pub value: Box<Option<Iota>>,
}

impl PatternIota {
    pub fn from_name(registry: &PatternRegistry, name: &str, value: Option<Iota>) -> PatternIota {
        PatternIota {
            signature: Signature::from_name(&registry, &name),
            value: Box::new(value),
        }
    }

    pub fn from_sig(name: &str, value: Option<Iota>) -> PatternIota {
        PatternIota {
            signature: Signature::from_sig(name),
            value: Box::new(value),
        }
    }
}

pub type Signature = Vec<PatternSigDir>;

#[derive(Debug, Clone, PartialEq)]
pub enum PatternSigDir {
    Q,
    A,
    S,
    D,
    E,
    W,
}

pub trait SignatureExt {
    fn from_sig(string: &str) -> Signature;
    fn from_name(registry: &PatternRegistry, string: &str) -> Signature;
    fn as_str(&self) -> String;
}

impl SignatureExt for Signature {
    fn from_sig(string: &str) -> Signature {
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

    fn from_name(registry: &PatternRegistry, string: &str) -> Signature {
        Signature::from_sig(&registry.find(&string.to_string()).unwrap().signature)
    }

    fn as_str(&self) -> String {
        self
            .iter()
            .map(|char| match char {
                PatternSigDir::Q => 'q',
                PatternSigDir::A => 'a',
                PatternSigDir::S => 's',
                PatternSigDir::D => 'd',
                PatternSigDir::E => 'e',
                PatternSigDir::W => 'w',
            })
            .collect()
    }
}
