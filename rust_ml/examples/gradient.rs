extern crate rust_ml;

use rust_ml::api;
use rust_ml::dataset::simple;
use rust_ml::gradient::gd;

fn main() {
    println!("Hello, gradients!");
    let simple = simple::read_data();
    let w = gd::gd(&simple, &gd::gradient, 0.00001, 0.01);
    println!("{:?}", w);
    let t0 = gd::classify(&simple.record_clone(0), &w);
    println!("{}", t0);
    let t1 = gd::classify(&simple.record_clone(1), &w);
    println!("{}", t1);
    let w = gd::gd(&simple, &gd::stochastic_gradient, 0.00001, 0.01);
    println!("{:?}", w);
    let t0 = gd::classify(&simple.record_clone(0), &w);
    println!("{}", t0);
    let t1 = gd::classify(&simple.record_clone(1), &w);
    println!("{}", t1);

}
