use std::rc::Rc;
mod number;
pub use number::*;
mod simple_type;
pub use simple_type::*;
mod boolean;
pub use boolean::*;
mod vecc;
pub use vecc::*;
mod sentence;
pub use sentence::*;
pub mod visitors;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolId<T>(pub String, pub Option<T>);


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Val<T>(pub Rc<T>);
impl<T> Val<T> {
    pub fn inner(&self) -> Rc<T> {
        Rc::clone(&self.0)
    }
    pub fn new(v: T) -> Self {
        Self(Rc::new(v))
    }
}

pub trait Visitor<T> {
    fn visit_sentence(&mut self, s: &Sentence) -> T;

    fn visit_arith(&mut self, s: &Arith) -> T;

    fn visit_boolf(&mut self, s: &BoolF) -> T;

    fn visit_val(&mut self, s: &Value) -> T;

    
}
