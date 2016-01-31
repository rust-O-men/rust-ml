extern crate rust_ml;
use rust_ml::criteria::gain;
use rust_ml::dataset::dataset_1;
use rust_ml::api;

fn main() {
	println!("Hello, ID3!");
	let data = dataset_1::read_data(); 
	let entropy = gain::information_gain(&data, &api::all_view(&data), &|rec| rec.boolean); 
	println!("{}", entropy); 
}
