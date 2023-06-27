pub struct Pattern {
    pub display_name: String,
    pub internal_name: String,
    pub signature: String,
    pub action: fn() -> ()
}

pub trait Operate {
    fn operate(&self) {}
}

