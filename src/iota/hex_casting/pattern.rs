use crate::interpreter::mishap::Mishap;
use crate::pattern_registry::PatternRegistryExt;
use crate::{parser::ActionValue, pattern_registry::PatternRegistry};

use crate::iota::Iota;

#[derive(Debug, Clone)]
pub struct PatternIota {
    pub signature: Signature,
    pub value: Box<Option<ActionValue>>,
    pub line: Option<(usize, usize)>
}

impl PatternIota {
    pub fn from_name(
        registry: &PatternRegistry,
        name: &str,
        value: Option<ActionValue>,
        line: Option<(usize, usize)>
    ) -> Result<PatternIota, Mishap> {
        Ok(PatternIota {
            signature: Signature::from_name(registry, name, &value)
                .ok_or(Mishap::InvalidPattern)?,
            value: Box::new(value),
            line
        })
    }

    pub fn from_sig(name: &str, value: Option<ActionValue>, line: Option<(usize, usize)>) -> PatternIota {
        PatternIota {
            signature: Signature::from_sig(name),
            value: Box::new(value),
            line
        }
    }
}

impl Iota for PatternIota {
    fn display(&self) -> String {
        let mut result = PatternRegistry::find(
            //todo: maybe make this not generate the entire registry each time
            &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
            &self.signature.as_str(),
            &self.value,
        )
        .map_or(self.signature.as_str(), |pat| pat.display_name);

        if let Some(value) = *self.value.clone() {
            match value {
                ActionValue::Iota(iota) => result = format!("{result}: {}", iota.display()),
                ActionValue::Bookkeeper(code) => result = format!("{result}: {code}"),
            }
        }
        result
    }
    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<PatternIota>() {
            Some(other) => self.signature == other.signature,
            None => false,
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
    ) -> Option<Signature>;
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
    ) -> Option<Signature> {
        Some(Signature::from_sig(
            &registry.find(string, value)?.signature,
        ))
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