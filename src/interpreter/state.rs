use std::collections::HashMap;

use crate::{
    iota::{
        BoolIota, EntityIota, GarbageIota, Iota, ListIota, NullIota, NumberIota, PatternIota,
        VectorIota,
    },
    pattern_registry::PatternRegistry,
};

use super::mishap::Mishap;

pub type Stack = Vec<Iota>;

pub type Considered = bool;
#[derive(Clone)]
pub struct State {
    pub stack: Stack,
    pub ravenmind: Option<Iota>,
    pub buffer: Option<Vec<(Iota, Considered)>>,
    pub heap: HashMap<String, i32>,
    pub pattern_registry: PatternRegistry,
    pub consider_next: bool,
}

pub enum Either<L, R> {
    L(L),
    R(R),
}

pub trait StackExt {
    fn get_number(&self, index: usize, arg_count: usize) -> Result<NumberIota, Mishap>;
    fn get_vector(&self, index: usize, arg_count: usize) -> Result<VectorIota, Mishap>;
    fn get_pattern(&self, index: usize, arg_count: usize) -> Result<PatternIota, Mishap>;
    fn get_bool(&self, index: usize, arg_count: usize) -> Result<BoolIota, Mishap>;
    fn get_garbage(&self, index: usize, arg_count: usize) -> Result<GarbageIota, Mishap>;
    fn get_null(&self, index: usize, arg_count: usize) -> Result<NullIota, Mishap>;
    fn get_entity(&self, index: usize, arg_count: usize) -> Result<EntityIota, Mishap>;
    fn get_list(&self, index: usize, arg_count: usize) -> Result<ListIota, Mishap>;

    fn get_num_or_vec(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<NumberIota, VectorIota>, Mishap>;
    fn get_iota(&self, index: usize, arg_count: usize) -> Result<&Iota, Mishap>;
}

impl StackExt for Stack {
    fn get_number(&self, index: usize, arg_count: usize) -> Result<NumberIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_vector(&self, index: usize, arg_count: usize) -> Result<VectorIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Vector(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_pattern(&self, index: usize, arg_count: usize) -> Result<PatternIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Pattern(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_bool(&self, index: usize, arg_count: usize) -> Result<BoolIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Bool(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_garbage(&self, index: usize, arg_count: usize) -> Result<GarbageIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Garbage(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_null(&self, index: usize, arg_count: usize) -> Result<NullIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Null(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_entity(&self, index: usize, arg_count: usize) -> Result<EntityIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Entity(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_list(&self, index: usize, arg_count: usize) -> Result<ListIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::List(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_num_or_vec(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<NumberIota, VectorIota>, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => Ok(Either::L(*x)),
            Iota::Vector(x) => Ok(Either::R(*x)),

            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_iota(&self, index: usize, arg_count: usize) -> Result<&Iota, Mishap> {
        let index = arg_count - index - 1;
        {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count - self.len()))
            } else {
                Ok(&self[self.len() - 1 - index])
            }
        }
    }
}
