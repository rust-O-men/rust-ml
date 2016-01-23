extern crate rust_ml;
use rust_ml::junk::minamoto::id3;
use rust_ml::junk::minamoto::entropy;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Category {
	A,
	B,
	C
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
enum Class {
	C1,
	C2
}

#[derive(Copy, Clone, Debug)]
struct Record {
    category: Category,
    number: u8,	
    boolean: bool,
}


fn main() {
    let c1 = Rc::new(Class::C1);
    let c2 = Rc::new(Class::C2);
    let data = vec![
	(Rc::new(Record{category: Category::A, number: 70, boolean: true}),  c1.clone()),
	(Rc::new(Record{category: Category::A, number: 90, boolean: true}), c2.clone()),
	(Rc::new(Record{category: Category::A, number: 85, boolean: false}), c2.clone()),
	(Rc::new(Record{category: Category::A, number: 95, boolean: false}), c2.clone()),
	(Rc::new(Record{category: Category::A, number: 70, boolean: false}), c1.clone()),
	(Rc::new(Record{category: Category::B, number: 90, boolean: true}), c1.clone()),
	(Rc::new(Record{category: Category::B, number: 78, boolean: false}), c1.clone()),
	(Rc::new(Record{category: Category::B, number: 65, boolean: true}), c1.clone()),
	(Rc::new(Record{category: Category::B, number: 75, boolean: false}), c1.clone()),
	(Rc::new(Record{category: Category::C, number: 80, boolean: true}), c2.clone()),
	(Rc::new(Record{category: Category::C, number: 70, boolean: true}), c2.clone()),
	(Rc::new(Record{category: Category::C, number: 80, boolean: false}), c1.clone()),
	(Rc::new(Record{category: Category::C, number: 80, boolean: false}), c1.clone()),
	(Rc::new(Record{category: Category::C, number: 96, boolean: false}), c1.clone())];

    let _c = id3::create_tree(&data,
                             &vec![Rc::new(r1)],
                             Rc::new(entropy::gain));
}


fn r1(r:&Record) -> bool { r.boolean }
