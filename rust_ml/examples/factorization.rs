extern crate nalgebra;
extern crate rust_ml;

use nalgebra::DMat;
use rust_ml::matrix::mf;

fn main() {
    println!("Hello, factorization!");
    let r = DMat::from_row_vec(8, 2, &vec![
        29.0, 29.0, 
        43.0, 33.0, 
        15.0, 25.0, 
        40.0, 28.0, 
        24.0, 11.0, 
        29.0, 29.0, 
        37.0, 23.0, 
        21.0, 6.0]
    );
    let (u, q) = mf::factorize(&r, 0.001, 1usize);
    println!("{:?}", u);
    println!("{:?}", q);
}


