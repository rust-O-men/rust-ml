extern crate rust_ml;
use rust_ml::junk::minamoto::entropy;

mod even_odd_class;

fn main() {
    let set = vec!(2i32,4i32,6i32, 8i32, 9i32);
    let classes = vec!(even_odd_class::EvenOddClass::Even, even_odd_class::EvenOddClass::Odd);
    println!("{}", entropy::entropy(&set, &classes));
    println!("gain odd {}", entropy::gain(&set, &classes, &even_odd_class::EvenOddClass::Odd));
}
