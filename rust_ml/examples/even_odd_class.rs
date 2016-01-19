extern crate rust_ml;
use rust_ml::junk::minamoto::entropy;

#[derive(Clone,Debug)]
pub enum EvenOddClass {
    Even,
    Odd
}

impl entropy::Classified<EvenOddClass> for i32 {
    fn class(&self) -> EvenOddClass {
        match self % 2 {
            0 => EvenOddClass::Even,
            _ => EvenOddClass::Odd
        }
    }
}

impl PartialEq for EvenOddClass {
    fn eq (&self, rhs:&EvenOddClass) -> bool {
        match (self, rhs) {
            (&EvenOddClass::Even, &EvenOddClass::Even) => true,
            (&EvenOddClass::Odd, &EvenOddClass::Odd) => true,
            (_, _) => false
        }
    }

    fn ne(&self, rhs:&EvenOddClass) -> bool {
        !self.eq(rhs)
    }
}
