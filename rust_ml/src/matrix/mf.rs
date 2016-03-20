extern crate nalgebra;

use self::nalgebra::DMat;

use super::super::api;

pub fn factorize(r: &DMat<api::Number>, step: api::Number, iterations: usize) -> (DMat<api::Number>, DMat<api::Number>) {
    let mut q = DMat::new_random(r.ncols(), r.ncols());
    let mut u = DMat::new_random(r.ncols(), r.nrows());
    for _ in 0..iterations {
        let current_q = q.clone();
        let current_u = u.clone();
        let q = current_q.clone() - (r.clone() - nalgebra::transpose(&current_u) * current_q.clone()) * step * current_u.clone();
        let u = current_u.clone() - (r.clone() - nalgebra::transpose(&current_u) * current_q.clone()) * step * current_q.clone();    
    }
    (q, u)
}