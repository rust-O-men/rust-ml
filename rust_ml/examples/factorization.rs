extern crate nalgebra;
extern crate rust_ml;

use nalgebra::DMatrix;
use rust_ml::matrix::mf;

fn main() {
    println!("Hello, matrix factorization!");
    let r = DMatrix::from_row_vector(5, 4, &vec![
        5.0, 3.0, 0.0, 1.0,
        4.0, 0.0, 0.0, 1.0,
        1.0, 1.0, 0.0, 5.0,
        1.0, 0.0, 0.0, 4.0,
        0.0, 1.0, 5.0, 4.0,

    ]);
    let (u, q) = mf::factorize(&r, 2usize, 0.0002, 5000usize, 0.02);
    println!("{:?}", u);
    println!("{:?}", q);
    println!("Original");
    println!("{:?}", r);
    let r_hat = u * nalgebra::transpose(&q);
    println!("Factorized");
    println!("{:?}", r_hat);
}


