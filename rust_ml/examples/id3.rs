extern crate rust_ml;
use rust_ml::criteria::gain;
use rust_ml::dataset::simple;
use rust_ml::trees::id3;
use rust_ml::api;

fn main() {
    println!("Hello, ID3!");
    let data = simple::read_data(); 
    let criterions: Vec<Box<api::Criterion<simple::Record>>> = vec![
        Box::new(|rec: &simple::Record| rec.boolean), 
        Box::new(|rec: &simple::Record| rec.category == simple::A_CATEGORY), 
        Box::new(|rec: &simple::Record| rec.category == simple::B_CATEGORY), 
        Box::new(|rec: &simple::Record| rec.category == simple::C_CATEGORY), 
        Box::new(|rec: &simple::Record| rec.number > 80.0)
    ];
    let tree = id3::id3(&data, &gain::information_gain, &criterions);
    let target = tree.apply(&data.records[1].0, &criterions);
    println!("{}", target);
}
