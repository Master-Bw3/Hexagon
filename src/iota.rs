#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum GarbageIota {
    Garbage,
}

#[derive(Debug, Clone)]
pub enum NullIota {
    Null,
}

#[derive(Debug, Clone)]
pub struct EntityIota {
    pub name: String,
    pub entity_type: String,
}