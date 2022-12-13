pub trait Trait {
    fn foo(&self);
}

impl<T: c::Trait> Trait for T {
    fn foo(&self) {}
}
