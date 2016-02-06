extern crate rust_ml;
use rust_ml::criteria::gain;
use rust_ml::dataset::simple;
use rust_ml::trees::id3;
use rust_ml::api;

fn main() {
	println!("Hello, ID3!");
	let data = simple::read_data(); 
	let boolean = |rec: &simple::Record| rec.boolean;
	let category_a = |rec: &simple::Record| rec.category == 0;
	let category_b = |rec: &simple::Record| rec.category == 1;
	let category_c = |rec: &simple::Record| rec.category == 2;
	let number = |rec: &simple::Record| rec.number > 80.0;
	let criterions :Vec<&api::Criterion<simple::Record>> = vec![
		&boolean, 
		&category_a, 
		&category_b, 
		&category_c, 
		&number
	];
	let tree = id3::id3(&data, &gain::information_gain, &criterions);
	let target = tree.apply(&data.records[1].0, &criterions);
	println!("{}", target);
}
