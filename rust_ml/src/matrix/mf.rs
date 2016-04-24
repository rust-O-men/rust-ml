extern crate nalgebra;

use self::nalgebra::{ DMatrix, Row, Column, Indexable };
use std::ops::Index;

use super::super::api;

pub fn factorize(r: &DMatrix<api::Number>, kk: usize, step: api::Number, iterations: usize, regularization: api::Number) -> (DMatrix<api::Number>, DMatrix<api::Number>) {
    let mut q = DMatrix::new_random(r.ncols(), kk);
    let mut u = DMatrix::new_random(r.nrows(), kk);
    q = nalgebra::transpose(&q);
    for _ in 0..iterations {
        for i in 0..r.nrows() {
            for j in 0..r.ncols() {
                let rij = *r.index((i, j));
                if rij > 0.0 {
                    let eij = rij - nalgebra::dot(&u.row(i), &q.column(j));
                    for k in 0..kk {
                        let uik = *u.index((i, k));
                        let qkj = *q.index((k, j));
                        let nuik = uik + step * (2.0 * eij * qkj - regularization * uik);
                        let nqkj = qkj + step * (2.0 * eij * uik - regularization * qkj);
                        unsafe {    
                            u.unsafe_set((i, k), nuik);
                            q.unsafe_set((k, j), nqkj);
                        }
                    }
                }
            }
        }
        let er = u.clone() * q.clone(); // TODO
        let mut e = 0.0;
        for i in 0..r.nrows() {
            for j in 0..r.ncols() {
                let rij = *r.index((i, j));
                if rij > 0.0 {
                    let de: f64 = rij - nalgebra::dot(&u.row(i), &q.column(j));
                    e += de.powf(2.0);
                    for k in 0..kk {
                        let uik: f64 = *u.index((i, k));
                        let qkj: f64 = *q.index((k, j));
                        e += regularization / 2.0 * (uik.powf(2.0) + qkj.powf(2.0));
                    }
                }
            }
        }
        if e < 0.01 {
            break;
        }
    }
    q = nalgebra::transpose(&q);
    (q, u)
}