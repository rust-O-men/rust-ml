extern crate rust_ml;
use rust_ml::criteria::gain;
use rust_ml::dataset::simple;
use rust_ml::api;

fn main() {
	println!("Hello, Entropy!");
	let data = simple::read_data(); 
	let entropy = gain::information_gain(&data, &api::all_view(&data), &|rec| rec.boolean); 
	println!("{}", entropy); 
}
