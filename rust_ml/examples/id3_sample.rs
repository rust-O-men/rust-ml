extern crate rust_ml;
use rust_ml::junk::minamoto::id3;

mod even_odd_class;
use even_odd_class::EvenOddClass as class;

fn main() {
    let set = vec!((2i32, class::Even), (4i32, class::Even), (6i32, class::Even), (8i32, class::Even), (9i32, class::Odd));
    let classes = vec!(class::Even, class::Odd);
    id3::create_tree(&set, &classes);
}
