pub struct Pattern {
    display_name: String,
    internal_name: String,
    signature: String,
    action: Fn
}

pub trait Operate {
    fn operate(&self) {}
}