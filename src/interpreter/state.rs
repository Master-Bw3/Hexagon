use std::{collections::HashMap, rc::Rc};

use crate::iota::{
    hex_casting::{
        number::NumberIota,
        pattern::{PatternIota, Signature},
        vector::VectorIota,
    },
    Iota,
};

use super::{continuation::ContinuationFrame, mishap::Mishap};

pub type Stack = Vec<Rc<dyn Iota>>;

pub type Considered = bool;

#[derive(Clone, Default)]
pub struct State {
    pub stack: Stack,
    pub ravenmind: Option<Rc<dyn Iota>>,
    pub entities: HashMap<String, Entity>,
    pub libraries: HashMap<[i32; 3], Library>,
    pub sentinal_location: Option<VectorIota>,
    pub buffer: Option<Vec<(Rc<dyn Iota>, Considered)>>,
    pub heap: HashMap<String, i32>,
    pub consider_next: bool,
    pub continuation: Vec<Rc<dyn ContinuationFrame>>,
}

pub type Library = HashMap<Signature, Rc<dyn Iota>>;

#[derive(Clone, Default, Debug)]
pub enum Holding {
    #[default]
    None,
    Focus(Option<Rc<dyn Iota>>),
    Trinket(Option<Rc<dyn Iota>>),
    Artifact(Option<Rc<dyn Iota>>),
    Cypher(Option<Rc<dyn Iota>>),
}

impl PartialEq for Holding {
    fn eq(&self, other: &Self) -> bool {
        fn eq(lhs: &Option<Rc<dyn Iota>>, rhs: &Option<Rc<dyn Iota>>) -> bool {
            match (lhs, rhs) {
                (Some(l), Some(r)) => l.tolerates_other(r),
                _ => false,
            }
        }
        match (self, other) {
            (Self::Focus(l0), Self::Focus(r0)) => eq(l0, r0),
            (Self::Trinket(l0), Self::Trinket(r0)) => eq(l0, r0),
            (Self::Artifact(l0), Self::Artifact(r0)) => eq(l0, r0),
            (Self::Cypher(l0), Self::Cypher(r0)) => eq(l0, r0),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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
    fn get_iota<T: Iota>(&self, index: usize, arg_count: usize) -> Result<Rc<T>, Mishap>;

    fn remove_args(&mut self, arg_count: &usize);
}

impl StackExt for Stack {
    // fn get_number(&self, index: usize, arg_count: usize) -> Result<NumberIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Number(x) => Ok(*x),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Number".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_vector(&self, index: usize, arg_count: usize) -> Result<VectorIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Vector(x) => Ok(*x),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Vector".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_pattern(&self, index: usize, arg_count: usize) -> Result<PatternIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Pattern(x) => Ok(x.clone()),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Pattern".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_bool(&self, index: usize, arg_count: usize) -> Result<BoolIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Bool(x) => Ok(*x),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Boolean".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_garbage(&self, index: usize, arg_count: usize) -> Result<GarbageIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Garbage(x) => Ok(x.clone()),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Garbage".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_null(&self, index: usize, arg_count: usize) -> Result<NullIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Null(x) => Ok(x.clone()),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Null".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_entity(&self, index: usize, arg_count: usize) -> Result<EntityIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Entity(x) => Ok(x.clone()),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Entity".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_list(&self, index: usize, arg_count: usize) -> Result<ListIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::List(x) => Ok(x.clone()),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "List".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_integer(&self, index: usize, arg_count: usize) -> Result<i32, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Number(x) => {
    //             if Iota::check_equality(&Iota::Number(*x), &Iota::Number(x.round())) {
    //                 Ok(x.round() as i32)
    //             } else {
    //                 Err(Mishap::IncorrectIota(
    //                     index,
    //                     "Integer".to_string(),
    //                     iota.clone(),
    //                 ))
    //             }
    //         }
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Integer".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_positive_integer_under_inclusive(
    //     &self,
    //     index: usize,
    //     list_size: usize,
    //     arg_count: usize,
    // ) -> Result<i32, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Number(x) => {
    //             if Iota::check_equality(&Iota::Number(*x), &Iota::Number(x.round()))
    //                 && (0..list_size).contains(&(x.round() as usize))
    //             {
    //                 Ok(x.round() as i32)
    //             } else {
    //                 Err(Mishap::IncorrectIota(
    //                     index,
    //                     "Positive Integer".to_string(),
    //                     iota.clone(),
    //                 ))
    //             }
    //         }
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Positive Integer".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_matrix(&self, index: usize, arg_count: usize) -> Result<MatrixIota, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Matrix(x) => Ok(x.clone()),
    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Matrix".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_num_or_vec(
    //     &self,
    //     index: usize,
    //     arg_count: usize,
    // ) -> Result<Either<NumberIota, VectorIota>, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Number(x) => Ok(Either::L(*x)),
    //         Iota::Vector(x) => Ok(Either::R(*x)),

    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Number or Vector".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_list_or_pattern_or_continuation(
    //     &self,
    //     index: usize,
    //     arg_count: usize,
    // ) -> Result<Either3<ListIota, PatternIota, ContinuationIota>, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::List(x) => Ok(Either3::L(x.clone())),
    //         Iota::Pattern(x) => Ok(Either3::M(x.clone())),
    //         Iota::Continuation(x) => Ok(Either3::R(x.clone())),

    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "List or Pattern".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_integer_or_list(
    //     &self,
    //     index: usize,
    //     arg_count: usize,
    // ) -> Result<Either<i32, ListIota>, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Number(x) => {
    //             if Iota::check_equality(&Iota::Number(*x), &Iota::Number(x.round())) {
    //                 Ok(Either::L(x.round() as i32))
    //             } else {
    //                 Err(Mishap::IncorrectIota(
    //                     index,
    //                     "Integer or List".to_string(),
    //                     iota.clone(),
    //                 ))
    //             }
    //         }
    //         Iota::List(x) => Ok(Either::R(x.clone())),

    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Integer or List".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    // fn get_num_or_vec_or_list(
    //     &self,
    //     index: usize,
    //     arg_count: usize,
    // ) -> Result<Either3<NumberIota, VectorIota, ListIota>, Mishap> {
    //     let iota = self.get_iota(index, arg_count)?;
    //     match iota {
    //         Iota::Number(x) => Ok(Either3::L(*x)),

    //         Iota::Vector(x) => Ok(Either3::M(*x)),

    //         Iota::List(x) => Ok(Either3::R(x.clone())),

    //         _ => Err(Mishap::IncorrectIota(
    //             index,
    //             "Number, Vector, or List".to_string(),
    //             iota.clone(),
    //         )),
    //     }
    // }

    fn get_iota<T: Iota>(&self, index: usize, arg_count: usize) -> Result<Rc<T>, Mishap> {
        let iota = {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count - self.len(), self.len()))?
            } else {
                self[(self.len() - arg_count) + index]
            }
        };

        iota.downcast_rc::<T>()
            .map_err(|_| Mishap::IncorrectIota(index, "Entity".to_string(), iota.clone()))
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
