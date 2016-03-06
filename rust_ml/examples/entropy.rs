extern crate rust_ml;
use rust_ml::criteria::gain;
use rust_ml::dataset::simple;
use rust_ml::api;

fn main() {
    println!("Hello, Entropy!");
    let data = simple::read_data(); 
    let view = api::DataSetView::full(data.len());
    let entropy = gain::information_gain(&data, &view, &|rec| rec.boolean); 
    println!("{}", entropy); 
}
