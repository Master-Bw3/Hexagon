#[derive(Debug)]
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

pub type NumberIota = f32;
pub type PatternIota = String;
pub type BoolIota = bool;
pub type ListIota = std::vec::Vec<Iota>;
pub type VectorIota = nalgebra::Matrix1x3<NumberIota>;

#[derive(Debug)]
pub enum GarbageIota {
    Garbage,
}

#[derive(Debug)]
pub enum NullIota {
    Null,
}

#[derive(Debug)]
pub struct EntityIota {
    pub name: String,
    pub entity_type: String,
}
