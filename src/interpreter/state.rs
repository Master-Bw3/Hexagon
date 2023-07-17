use std::collections::HashMap;

use crate::{iota::{
    BoolIota, EntityIota, GarbageIota, Iota, ListIota, MatrixIota, NullIota, NumberIota,
    PatternIota, Signature, VectorIota, ContinuationIota,
}, pattern_registry::PatternRegistry, parser::AstNode};

use super::mishap::Mishap;

pub type Stack = Vec<Iota>;

pub type Considered = bool;

#[derive(Clone, Default)]
pub struct State {
    pub stack: Stack,
    pub ravenmind: Option<Iota>,
    pub entities: HashMap<String, Entity>,
    pub libraries: HashMap<[i32; 3], Library>,
    pub sentinal_location: Option<VectorIota>,
    pub buffer: Option<Vec<(Iota, Considered)>>,
    pub heap: HashMap<String, i32>,
    pub consider_next: bool,
    pub continuation: Vec<AstNode>,
}

pub type Library = HashMap<Signature, Iota>;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum Holding {
    #[default]
    None,
    Focus(Option<Iota>),
    Trinket(Option<Iota>),
    Artifact(Option<Iota>),
    Cypher(Option<Iota>),
}

pub enum Either<L, R> {
    L(L),
    R(R),
}

pub enum Either3<L, M, R> {
    L(L),
    M(M),
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
    fn get_matrix(&self, index: usize, arg_count: usize) -> Result<MatrixIota, Mishap>;

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

    fn get_list_or_pattern_or_continuation(
        &self,
        index: usize,
        arg_count: usize,
    ) ->Result<Either3<ListIota, PatternIota, ContinuationIota>, Mishap>;

    fn get_integer_or_list(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<i32, ListIota>, Mishap>;

    fn get_num_or_vec_or_list(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either3<NumberIota, VectorIota, ListIota>, Mishap>;

    fn get_iota(&self, index: usize, arg_count: usize) -> Result<&Iota, Mishap>;

    fn remove_args(&mut self, arg_count: &usize);
}

impl StackExt for Stack {
    fn get_number(&self, index: usize, arg_count: usize) -> Result<NumberIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(
                index,
                "Number".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_vector(&self, index: usize, arg_count: usize) -> Result<VectorIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Vector(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(
                index,
                "Vector".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_pattern(&self, index: usize, arg_count: usize) -> Result<PatternIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Pattern(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(
                index,
                "Pattern".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_bool(&self, index: usize, arg_count: usize) -> Result<BoolIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Bool(x) => Ok(*x),
            _ => Err(Mishap::IncorrectIota(
                index,
                "Boolean".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_garbage(&self, index: usize, arg_count: usize) -> Result<GarbageIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Garbage(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(
                index,
                "Garbage".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_null(&self, index: usize, arg_count: usize) -> Result<NullIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Null(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(
                index,
                "Null".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_entity(&self, index: usize, arg_count: usize) -> Result<EntityIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Entity(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(
                index,
                "Entity".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_list(&self, index: usize, arg_count: usize) -> Result<ListIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::List(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(
                index,
                "List".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_integer(&self, index: usize, arg_count: usize) -> Result<i32, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => {
                if Iota::check_equality(&Iota::Number(*x), &Iota::Number(x.round())) {
                    Ok(x.round() as i32)
                } else {
                    Err(Mishap::IncorrectIota(
                        index,
                        "Integer".to_string(),
                        iota.clone(),
                    ))
                }
            }
            _ => Err(Mishap::IncorrectIota(
                index,
                "Integer".to_string(),
                iota.clone(),
            )),
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
                    Err(Mishap::IncorrectIota(
                        index,
                        "Positive Integer".to_string(),
                        iota.clone(),
                    ))
                }
            }
            _ => Err(Mishap::IncorrectIota(
                index,
                "Positive Integer".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_matrix(&self, index: usize, arg_count: usize) -> Result<MatrixIota, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Matrix(x) => Ok(x.clone()),
            _ => Err(Mishap::IncorrectIota(
                index,
                "Matrix".to_string(),
                iota.clone(),
            )),
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

            _ => Err(Mishap::IncorrectIota(
                index,
                "Number or Vector".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_list_or_pattern_or_continuation(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either3<ListIota, PatternIota, ContinuationIota>, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::List(x) => Ok(Either3::L(x.clone())),
            Iota::Pattern(x) => Ok(Either3::M(x.clone())),
            Iota::Continuation(x) => Ok(Either3::R(x.clone())),

            _ => Err(Mishap::IncorrectIota(
                index,
                "List or Pattern".to_string(),
                iota.clone(),
            )),
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
                    Err(Mishap::IncorrectIota(
                        index,
                        "Integer or List".to_string(),
                        iota.clone(),
                    ))
                }
            }
            Iota::List(x) => Ok(Either::R(x.clone())),

            _ => Err(Mishap::IncorrectIota(
                index,
                "Integer or List".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_num_or_vec_or_list(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either3<NumberIota, VectorIota, ListIota>, Mishap> {
        let iota = self.get_iota(index, arg_count)?;
        match iota {
            Iota::Number(x) => Ok(Either3::L(*x)),

            Iota::Vector(x) => Ok(Either3::M(x.clone())),

            Iota::List(x) => Ok(Either3::R(x.clone())),

            _ => Err(Mishap::IncorrectIota(
                index,
                "Number, Vector, or List".to_string(),
                iota.clone(),
            )),
        }
    }

    fn get_iota(&self, index: usize, arg_count: usize) -> Result<&Iota, Mishap> {
        {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count - self.len(), self.len()))
            } else {
                Ok(&self[(self.len() - arg_count) + index])
            }
        }
    }

    fn remove_args(&mut self, arg_count: &usize) {
        self.drain((self.len() - arg_count)..);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub holding: Box<Holding>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Animal,
    Monster,
    Living,
    Item,
    Player,
    Misc,
}

impl EntityType {
    pub fn display(&self) -> String {
        match self {
            EntityType::Animal => "Animal".to_string(),
            EntityType::Monster => "Monster".to_string(),
            EntityType::Living => "Living".to_string(),
            EntityType::Item => "Item".to_string(),
            EntityType::Player => "Player".to_string(),
            EntityType::Misc => "Misc".to_string(),
        }
    }
}
