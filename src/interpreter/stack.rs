use crate::iota::{
    BoolIota, EntityIota, GarbageIota, Iota, ListIota, NullIota, NumberIota, PatternIota,
    VectorIota,
};

use super::mishap::Mishap;

pub type Stack = Vec<Iota>;

pub struct State {
    pub stack: Stack,
    pub ravenmind: Option<Iota>,
}

pub trait StackExt {
    fn get_number_iota(&self, index: usize) -> Result<NumberIota, Mishap>;
    fn get_vector_iota(&self, index: usize) -> Result<VectorIota, Mishap>;
    fn get_pattern_iota(&self, index: usize) -> Result<PatternIota, Mishap>;
    fn get_bool_iota(&self, index: usize) -> Result<BoolIota, Mishap>;
    fn get_garbage_iota(&self, index: usize) -> Result<GarbageIota, Mishap>;
    fn get_null_iota(&self, index: usize) -> Result<NullIota, Mishap>;
    fn get_entity_iota(&self, index: usize) -> Result<EntityIota, Mishap>;
    fn get_list_iota(&self, index: usize) -> Result<ListIota, Mishap>;
}

impl StackExt for Stack {
    fn get_number_iota(&self, index: usize) -> Result<NumberIota, Mishap> {
        let iota = self.get(index).ok_or(Mishap::NotEnoughIotas(1))?;
        match iota {
            Iota::Number(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_vector_iota(&self, index: usize) -> Result<VectorIota, Mishap> {
        let iota = self.get(index).ok_or(Mishap::NotEnoughIotas(1))?;
        match iota {
            Iota::Vector(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_pattern_iota(&self, index: usize) -> Result<PatternIota, Mishap> {
        let iota = self.get(index).ok_or(Mishap::NotEnoughIotas(1))?;
        match iota {
            Iota::Pattern(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_bool_iota(&self, index: usize) -> Result<BoolIota, Mishap> {
        let iota = self.get(index).ok_or(Mishap::NotEnoughIotas(1))?;
        match iota {
            Iota::Bool(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_garbage_iota(&self, index: usize) -> Result<GarbageIota, Mishap> {
        let iota = self.get(index).ok_or(Mishap::NotEnoughIotas(1))?;
        match iota {
            Iota::Garbage(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_null_iota(&self, index: usize) -> Result<NullIota, Mishap> {
        let iota = self.get(index).ok_or(Mishap::NotEnoughIotas(1))?;
        match iota {
            Iota::Null(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_entity_iota(&self, index: usize) -> Result<EntityIota, Mishap> {
        let iota = self.get(index).ok_or(Mishap::NotEnoughIotas(1))?;
        match iota {
            Iota::Entity(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_list_iota(&self, index: usize) -> Result<ListIota, Mishap> {
        let iota = self.get(index).ok_or(Mishap::NotEnoughIotas(1))?;
        match iota {
            Iota::List(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }
}
