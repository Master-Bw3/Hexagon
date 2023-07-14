use std::fmt::Debug;
use std::{collections::HashMap, ops::Not};

use nalgebra::{DMatrix, Dyn, Matrix};

use crate::{
    interpreter::state::{Entity, EntityType},
    parser::ActionValue,
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Iota {
    //Hex Casting
    Number(NumberIota),
    Vector(VectorIota),
    Pattern(PatternIota),
    Bool(BoolIota),
    Garbage(GarbageIota),
    Null(NullIota),
    Entity(EntityIota),
    List(ListIota),
    //MoreIotas
    Matrix(MatrixIota),
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

    pub fn display_type(&self) -> &'static str {
        match self {
            Iota::Number(_) => "Number",
            Iota::Vector(_) => "Vector",
            Iota::Pattern(_) => "Pattern",
            Iota::Bool(_) => "Bool",
            Iota::Garbage(_) => "Garbage",
            Iota::Null(_) => "Null",
            Iota::Entity(_) => "Entity",
            Iota::List(_) => "List",
            Iota::Matrix(_) => "Matrix",
        }
    }

    pub fn display(&self) -> String {
        match self {
            Iota::Number(num) => num.display(),
            Iota::Vector(vec) => vec.display(),
            Iota::Pattern(pat) => pat.display(),
            Iota::Bool(bool) => bool.display(),
            Iota::Garbage(garbage) => garbage.display(),
            Iota::Null(null) => null.display(),
            Iota::Entity(name) => name.display(),
            Iota::List(list) => list.display(),
            Iota::Matrix(matrix) => matrix.display(),
        }
    }
}

pub trait Display {
    fn display(&self) -> String;
}

pub type NumberIota = f32;

impl Display for NumberIota {
    fn display(&self) -> String {
        self.to_string()
    }
}

pub type BoolIota = bool;

impl Display for BoolIota {
    fn display(&self) -> String {
        self.to_string()
    }
}

pub type ListIota = std::vec::Vec<Iota>;

impl Display for ListIota {
    fn display(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(Iota::display)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub type VectorIota = nalgebra::Matrix1x3<NumberIota>;

impl Display for VectorIota {
    fn display(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GarbageIota {
    Garbage,
}

impl Display for GarbageIota {
    fn display(&self) -> String {
        "Garbage".to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NullIota {
    Null,
}

impl Display for NullIota {
    fn display(&self) -> String {
        "Null".to_string()
    }
}

//reference to an entity
pub type EntityIota = String;

impl Display for EntityIota {
    fn display(&self) -> String {
        format!("@{self}")
    }
}

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

impl Display for PatternIota {
    fn display(&self) -> String {
        PatternRegistry::find(
            //todo: maybe make this not generate the entire registry each time
            &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
            &self.signature.as_str(),
            &None,
        )
        .map_or(self.signature.as_str(), |pat| pat.display_name)
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

pub type MatrixIota = Matrix<NumberIota, Dyn, Dyn, nalgebra::VecStorage<NumberIota, Dyn, Dyn>>;

impl Display for MatrixIota {
    fn display(&self) -> String {
        let mut out = vec![];
        for row in self.row_iter() {
            let row_out = row.iter().map(f32::to_string).collect::<Vec<_>>();
            let row_str = format!("[{}]", row_out.join(", "));
            out.push(row_str)
        }
        out.join("\n")
    }
}
