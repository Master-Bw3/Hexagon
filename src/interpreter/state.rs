use std::{collections::HashMap, default, ops::Deref, rc::Rc, sync::Arc};

use im::{vector, Vector};

use crate::{
    iota::{
        hex_casting::{continuation, pattern::Signature, vector::VectorIota, entity::EntityIota},
        Iota,
    },
    parser::{AstNode, Macros},
    pattern_registry::PatternRegistry,
};

use super::{
    continuation::{ContinuationFrame, ContinuationFrameTrait, FrameEvaluate},
    interpret_node,
    mishap::Mishap,
};

pub type Stack = Vector<Rc<dyn Iota>>;

pub type Considered = bool;

#[derive(Clone, Default, Debug)]
pub struct State {
    pub stack: Stack,
    pub ravenmind: Option<Rc<dyn Iota>>,
    pub entities: HashMap<String, Entity>,
    pub libraries: HashMap<[i32; 3], Library>,
    pub sentinal_location: Option<VectorIota>,
    pub buffer: Option<Vector<(Rc<dyn Iota>, Considered)>>,
    pub heap: HashMap<String, i32>,
    pub consider_next: bool,
    pub continuation: Vector<ContinuationFrame>,
    pub wisps: Vector<Wisp>,
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
                (Some(l), Some(r)) => l.tolerates_other(r.deref()),
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

type Either3Rc<L, M, R> = Either3<Rc<L>, Rc<M>, Rc<R>>;

pub trait StackExt {
    fn get_iota<T: Iota>(&self, index: usize, arg_count: usize) -> Result<Rc<T>, Mishap>;

    fn get_any_iota(&self, index: usize, arg_count: usize) -> Result<Rc<dyn Iota>, Mishap>;

    fn get_iota_a_or_b<T: Iota, U: Iota>(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<Rc<T>, Rc<U>>, Mishap>;
    fn get_iota_a_b_or_c<T: Iota, U: Iota, V: Iota>(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either3Rc<T, U, V>, Mishap>;

    fn remove_args(&mut self, arg_count: &usize);
}

impl StackExt for Stack {
    fn get_iota<T: Iota>(&self, index: usize, arg_count: usize) -> Result<Rc<T>, Mishap> {
        let iota = {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count, self.len()))?
            } else {
                self[(self.len() - arg_count) + index].to_owned()
            }
        };

        iota.clone()
            .downcast_rc::<T>()
            .map_err(|_| Mishap::IncorrectIota(index, T::display_type_name(), iota.clone()))
    }

    fn get_any_iota(&self, index: usize, arg_count: usize) -> Result<Rc<dyn Iota>, Mishap> {
        let iota = {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count, self.len()))?
            } else {
                self[(self.len() - arg_count) + index].to_owned()
            }
        };

        Ok(iota)
    }

    fn remove_args(&mut self, arg_count: &usize) {
        self.slice((self.len() - arg_count)..);
    }

    fn get_iota_a_or_b<T: Iota, U: Iota>(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either<Rc<T>, Rc<U>>, Mishap> {
        let iota = {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count, self.len()))?
            } else {
                self[(self.len() - arg_count) + index].to_owned()
            }
        };

        let left = iota.clone().downcast_rc::<T>();
        let right = iota.clone().downcast_rc::<U>();

        match (left, right) {
            (Ok(l), Err(_)) => Ok(Either::L(l)),
            (Err(_), Ok(r)) => Ok(Either::R(r)),
            (Err(_), Err(_)) => Err(Mishap::IncorrectIota(
                index,
                format!("{} or {}", T::display_type_name(), U::display_type_name()),
                iota.clone(),
            )),
            _ => unreachable!(),
        }
    }

    fn get_iota_a_b_or_c<T: Iota, U: Iota, V: Iota>(
        &self,
        index: usize,
        arg_count: usize,
    ) -> Result<Either3<Rc<T>, Rc<U>, Rc<V>>, Mishap> {
        let iota = {
            if self.len() < arg_count {
                Err(Mishap::NotEnoughIotas(arg_count, self.len()))?
            } else {
                self[(self.len() - arg_count) + index].to_owned()
            }
        };

        let left = iota.clone().downcast_rc::<T>();
        let middle = iota.clone().downcast_rc::<U>();

        let right = iota.clone().downcast_rc::<V>();

        match (left, middle, right) {
            (Ok(l), Err(_), Err(_)) => Ok(Either3::L(l)),
            (Err(_), Ok(m), Err(_)) => Ok(Either3::M(m)),
            (Err(_), Err(_), Ok(r)) => Ok(Either3::R(r)),
            (Err(_), Err(_), Err(_)) => Err(Mishap::IncorrectIota(
                index,
                format!(
                    "{}, {} or {}",
                    T::display_type_name(),
                    U::display_type_name(),
                    V::display_type_name()
                ),
                iota.clone(),
            )),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub name: String,
    pub uuid: String,
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
    Wisp,
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
            EntityType::Wisp => "Wisp".to_string(),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Wisp {
    pub stack: Stack,
    pub ravenmind: Option<Rc<dyn Iota>>,
    pub heap: HashMap<String, i32>,
    pub code: Vector<AstNode>,
    pub self_ref: Option<EntityIota>,
}

impl Wisp {
    pub fn evaluate(
        &self,
        main_state: &mut State,
        pattern_registry: &PatternRegistry,
        macros: &Macros,
    ) -> Result<Wisp, (Mishap, (usize, usize))> {
        let mut wisp_state = State {
            stack: self.stack.clone(),
            ravenmind: self.ravenmind.clone(),
            heap: self.heap.clone(),
            buffer: Default::default(),
            consider_next: Default::default(),
            continuation: Default::default(),
            ..main_state.clone()
        };

        if let Some(entity) = self.self_ref.clone() {
            wisp_state.stack.push_back(Rc::new(entity))
        }

        wisp_state
            .continuation
            .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                nodes_queue: Vector::from(self.code.clone()),
            }));

        while !wisp_state.continuation.is_empty() {
            //get top frame and remove it from the stack
            let frame = wisp_state.continuation.pop_back().unwrap();
            //evaluate the top frame (mutates state)
            frame.evaluate(&mut wisp_state, pattern_registry, macros)?;
        }

        main_state.entities = wisp_state.entities.clone();
        main_state.libraries = wisp_state.libraries.clone();
        main_state.sentinal_location = wisp_state.sentinal_location;
        main_state.wisps = wisp_state.wisps;

        //set self_ref to None so that it isn't added to stack in future iterations
        let result = Wisp {
            stack: wisp_state.stack,
            ravenmind: wisp_state.ravenmind,
            heap: wisp_state.heap,
            code: self.code.clone(),
            self_ref: None,
        };

        Ok(result)
    }
}
