pub struct Pattern {
    display_name: String,
    internal_name: String,
    signature: String,
    action: fn() -> ()
}

pub trait Operate {
    fn operate(&self) {}
}