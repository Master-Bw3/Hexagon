use std::collections::HashMap;

use crate::iota::{
    BoolIota, EntityIota, GarbageIota, Iota, ListIota, NullIota, NumberIota, PatternIota,
    VectorIota,
};

use super::mishap::Mishap;

pub type Stack = Vec<Iota>;

pub type Considered = bool;

#[derive(Clone, Default)]
pub struct State {
    pub stack: Stack,
    pub ravenmind: Option<Iota>,
    pub offhand: Offhand,
    pub buffer: Option<Vec<(Iota, Considered)>>,
    pub heap: HashMap<String, i32>,
    pub consider_next: bool,
    pub halt: bool,
}


#[derive(Clone, Default)]
pub enum Offhand {
    #[default]
    None,
    Focus
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

    fn get_integer(&self, index: usize, arg_count: usize) -> Result<i32, Mishap>;
    fn get_positive_integer_under_inclusive(
        &self,
        index: usize,
        list_size: usize,
        arg_count: usize,
    ) -> Result<i32, Mishap>;

    fn get_num_or_vec(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<NumberIota, VectorIota>, Mishap>;

    fn get_list_or_pattern(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<ListIota, PatternIota>, Mishap>;

    fn get_integer_or_list(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<i32, ListIota>, Mishap>;

    fn get_iota(&self, index: usize, arg_count: usize) -> Result<&Iota, Mishap>;

    fn remove_args(&mut self, arg_count: &usize);
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

    fn get_integer(&self, index: usize, arg_count: usize) -> Result<i32, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => {
                if Iota::check_equality(&Iota::Number(*x), &Iota::Number(x.round())) {
                    Ok(x.round() as i32)
                } else {
                    Err(Mishap::IncorrectIota(index))
                }
            }
            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_positive_integer_under_inclusive(
        &self,
        index: usize,
        list_size: usize,
        arg_count: usize,
    ) -> Result<i32, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => {
                if Iota::check_equality(&Iota::Number(*x), &Iota::Number(x.round()))
                    && (0..list_size).contains(&(x.round() as usize))
                {
                    Ok(x.round() as i32)
                } else {
                    Err(Mishap::IncorrectIota(index))
                }
            }
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

    fn get_list_or_pattern(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<ListIota, PatternIota>, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::List(x) => Ok(Either::L(x.clone())),
            Iota::Pattern(x) => Ok(Either::R(x.clone())),

            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_integer_or_list(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<i32, ListIota>, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => {
                if Iota::check_equality(&Iota::Number(*x), &Iota::Number(x.round())) {
                    Ok(Either::L(x.round() as i32))
                } else {
                    Err(Mishap::IncorrectIota(index))
                }
            }
            Iota::List(x) => Ok(Either::R(x.clone())),

            _ => Err(Mishap::IncorrectIota(index)),
        }
    }

    fn get_iota(&self, index: usize, arg_count: usize) -> Result<&Iota, Mishap> {
        {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count - self.len()))
            } else {
                Ok(&self[(self.len() - arg_count) + index])
            }
        }
    }

    fn remove_args(&mut self, arg_count: &usize) {
        self.drain((self.len() - arg_count)..);
    }
}
