use std::{collections::HashMap, ops::Not};

use crate::{
    interpreter::state::{Entity, EntityType, Holding},
    parse_config::Config,
    parser::ActionValue,
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

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

impl Iota {
    pub fn check_equality(&self, other: &Iota) -> bool {
        let tolerance = 0.001;

        match (self, other) {
            (Iota::Number(a), Iota::Number(b)) => (a - b).abs() < tolerance,
            (Iota::Vector(a), Iota::Vector(b)) => (a.norm() - b.norm()).abs() < tolerance,
            (Iota::Pattern(a), Iota::Pattern(b)) => a.signature == b.signature,
            (Iota::Bool(a), Iota::Bool(b)) => a == b,
            (Iota::Garbage(_), Iota::Garbage(_)) => true,
            (Iota::Null(_), Iota::Null(_)) => true,
            (Iota::Entity(a), Iota::Entity(b)) => a == b,
            (Iota::List(a), Iota::List(b)) => (a
                .iter()
                .zip(b.iter())
                .map(|(a, b)| Iota::check_equality(a, b)))
            .collect::<Vec<bool>>()
            .contains(&false)
            .not(),
            _ => false,
        }
    }

    pub fn is_entity_list(
        &self,
        entity_type: Option<&EntityType>,
        entities: &HashMap<String, Entity>,
    ) -> bool {
        match self {
            Iota::List(x) => {
                x.iter()
                    .filter(|i| i.is_entity(entity_type, entities))
                    .collect::<Vec<&Iota>>()
                    .len()
                    == x.len()
            }
            _ => false,
        }
    }

    pub fn is_entity(
        &self,
        entity_type: Option<&EntityType>,
        entities: &HashMap<String, Entity>,
    ) -> bool {
        match self {
            Iota::Entity(entity_name) => match entities.get(entity_name) {
                Some(entity) => match entity_type {
                    Some(t) => entity.entity_type == *t,
                    None => true,
                },
                None => false,
            },

            _ => false,
        }
    }
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

//reference to an entity
pub type EntityIota = String;

#[derive(Debug, Clone, PartialEq)]
pub struct PatternIota {
    pub signature: Signature,
    pub value: Box<Option<ActionValue>>,
}

impl PatternIota {
    pub fn from_name(
        registry: &PatternRegistry,
        name: &str,
        value: Option<ActionValue>,
    ) -> PatternIota {
        PatternIota {
            signature: Signature::from_name(registry, name, &value),
            value: Box::new(value),
        }
    }

    pub fn from_sig(name: &str, value: Option<ActionValue>) -> PatternIota {
        PatternIota {
            signature: Signature::from_sig(name),
            value: Box::new(value),
        }
    }
}

pub type Signature = Vec<PatternSigDir>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn from_name(
        registry: &PatternRegistry,
        string: &str,
        value: &Option<ActionValue>,
    ) -> Signature;
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
                _ => panic!("invalid signature: {}", string),
            })
            .collect()
    }

    fn from_name(
        registry: &PatternRegistry,
        string: &str,
        value: &Option<ActionValue>,
    ) -> Signature {
        Signature::from_sig(&registry.find(string, value).expect(string).signature)
    }

    fn as_str(&self) -> String {
        self.iter()
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
