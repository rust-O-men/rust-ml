extern crate rust_ml;
use rust_ml::junk::minamoto::entropy;

mod even_odd_class;

fn main() {

    let set = vec!((2i32, even_odd_class::EvenOddClass::Even),
                   (4i32, even_odd_class::EvenOddClass::Even),
                   (6i32, even_odd_class::EvenOddClass::Even),
                   (8i32, even_odd_class::EvenOddClass::Even),
                   (9i32, even_odd_class::EvenOddClass::Odd));

    println!("2 {}", entropy::entropy(&set));
    println!("gain2 odd {}", entropy::gain(&set, even_odd_class::EvenOddClass::Odd));
}
