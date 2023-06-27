use crate::iota::{
    BoolIota, EntityIota, GarbageIota, Iota, ListIota, NullIota, NumberIota, PatternIota,
    VectorIota,
};

use super::mishap::Mishap;

pub type Stack = Vec<Iota>;

#[derive(Debug, Clone)]

pub struct State {
    pub stack: Stack,
    pub ravenmind: Option<Iota>,
}

pub enum Either<A, B> {
    A(A),
    B(B),
}

pub trait StackExt {
    fn get_number(&mut self, index: usize, arg_count: usize) -> Result<NumberIota, Mishap>;
    fn get_vector(&mut self, index: usize, arg_count: usize) -> Result<VectorIota, Mishap>;
    fn get_pattern(&mut self, index: usize, arg_count: usize) -> Result<PatternIota, Mishap>;
    fn get_bool(&mut self, index: usize, arg_count: usize) -> Result<BoolIota, Mishap>;
    fn get_garbage(&mut self, index: usize, arg_count: usize) -> Result<GarbageIota, Mishap>;
    fn get_null(&mut self, index: usize, arg_count: usize) -> Result<NullIota, Mishap>;
    fn get_entity(&mut self, index: usize, arg_count: usize) -> Result<EntityIota, Mishap>;
    fn get_list(&mut self, index: usize, arg_count: usize) -> Result<ListIota, Mishap>;

    fn get_num_or_vec(&mut self, index: usize, arg_count: usize) -> Result<Either<NumberIota, VectorIota>, Mishap>;
    fn get_iota(&mut self, index: usize, arg_count: usize) -> Result<Iota, Mishap>;

}

impl StackExt for Stack {
    fn get_number(&mut self, index: usize, arg_count: usize) -> Result<NumberIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => Ok(x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_vector(&mut self, index: usize, arg_count: usize) -> Result<VectorIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Vector(x) => Ok(x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_pattern(&mut self, index: usize, arg_count: usize) -> Result<PatternIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Pattern(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_bool(&mut self, index: usize, arg_count: usize) -> Result<BoolIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Bool(x) => Ok(x),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_garbage(&mut self, index: usize, arg_count: usize) -> Result<GarbageIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Garbage(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_null(&mut self, index: usize, arg_count: usize) -> Result<NullIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Null(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_entity(&mut self, index: usize, arg_count: usize) -> Result<EntityIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Entity(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_list(&mut self, index: usize, arg_count: usize) -> Result<ListIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::List(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_num_or_vec(&mut self, index: usize, arg_count: usize) -> Result<Either<NumberIota, VectorIota>, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => Ok(Either::A(x)),
            Iota::Vector(x) => Ok(Either::B(x)),

            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_iota(&mut self, index: usize, arg_count: usize) -> Result<Iota, Mishap> {
        let index = arg_count - index - 1;
        {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count - self.len()))
            } else {
                Ok(self.remove(self.len() - 1 - index))
            }
        }
    }

}

