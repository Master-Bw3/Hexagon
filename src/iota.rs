#[derive(Debug)]
pub enum Iota<'a> {
    Number(f32),
    Vector(f32, f32, f32),
    Pattern(String),
    Bool(bool),
    Garbage,
    Null,
    Entity { name: &'a str, entity_type: &'a str },
    List(std::vec::Vec<Iota<'a>>),
}
