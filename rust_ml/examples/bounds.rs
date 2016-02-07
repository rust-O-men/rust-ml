extern crate rust_ml;

use rust_ml::api;
use rust_ml::features::bounds;
use rust_ml::dataset::iris_flower;

fn main() {
	println!("Hello, bounds!");
	let data = iris_flower::read_data(); 
	let bounds1 = bounds::uniform_selector(&data, 0 as api::Feature, 5usize);
	println!("{:?}", bounds1);
	let _ = bounds::bounds_criterions(&data, &bounds::uniform_selector, 5usize);
}

