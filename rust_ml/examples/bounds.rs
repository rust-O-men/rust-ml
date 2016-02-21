extern crate rust_ml;

use rust_ml::api;
use rust_ml::features::bounds;
use rust_ml::features::vectorize;
use rust_ml::dataset::iris_flower;
use rust_ml::dataset::simple;

fn main() {
    println!("Hello, bounds and vectorize!");
    let iris = iris_flower::read_data(); 
    let bounds1 = bounds::uniform_selector(&iris, 0 as api::Feature, 5usize);
    println!("{:?}", bounds1);
    let _ = bounds::bounds_criterions(&iris, &bounds::uniform_selector, 5usize);
    for record in iris.records.iter() {
        let vectorized = vectorize::vectorize(&record.0);
        println!("{:?}", vectorized);
    }
    let simple = simple::read_data();
    for record in simple.records.iter() {
        let vectorized = vectorize::vectorize(&record.0);
        println!("{:?}", vectorized);
    }
}

