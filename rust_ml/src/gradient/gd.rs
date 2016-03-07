extern crate rand;

use self::rand::{thread_rng, Rng};

use super::super::api;
use super::super::features::vectorize;


pub type Parameters = Vec<api::Number>;

pub type Gradient<T> = Fn(&api::DataSet<T>) -> Parameters;


pub fn classify<T: api::RecordMeta>(record: &T, params: &Parameters) -> api::Target {
    let v = f(record, params);
    if v < 0.0 {
        0
    } else {
        1
    }
}


fn f<T: api::RecordMeta>(record: &T, params: &Parameters) -> api::Number {
    let x = vectorize::vectorize(record, true);
    x.iter().zip(params.iter()).fold(0.0, |acc, (&x, &w)| acc + x * w)
}


fn normalize_target(t: api::Target) -> api::Number {
    match t {
        0 => -1.0,
        1 => 1.0,
        _ => panic!("Not a binary target!")
    }
}


fn q<T: api::RecordMeta>(ds: &api::DataSet<T>, params: &Parameters) -> api::Number {
    let m = ds.records().map(|ref rec| f(&rec.0, params) * normalize_target(rec.1));
    m.map(|m| if m < 0.0 {m} else {0.0}).fold(0.0, |acc, mi| acc + mi)
}


pub fn gradient<T: api::RecordMeta>(ds: &api::DataSet<T>) -> Parameters { 
    let mut result = vec![0.0; vectorize::vector_len(ds) + 1];
    for record in ds.records() {
        let x = vectorize::vectorize(&record.0, true);
        let y = normalize_target(record.1);
        for (i, xi) in x.iter().enumerate() {
            result[i] -= xi * y;
        }
    }
    result
}


pub fn stochastic_gradient<T: api::RecordMeta>(ds: &api::DataSet<T>) -> Parameters { 
    let mut rng = thread_rng();
    let rec_no = rng.gen_range(0usize, ds.len());
    let mut found = None;
    for (i, rec) in ds.records().enumerate() {
        if i == rec_no {
            found = Some(rec);
            break;
        }
    };
    let record = found.unwrap();
    let x = vectorize::vectorize(&record.0, true);
    let y = normalize_target(record.1);
    x.iter().map(|xi| -y * xi).collect()
}


fn distance(a: &Parameters, b: &Parameters) -> api::Number {
    a.iter().zip(b.iter()).fold(0.0, |acc, (&ai, &bi)| acc + (ai - bi).powf(2.0)).sqrt()
}


fn next_parameters(old: &Parameters, new: &Parameters, step: api::Number) -> Parameters {
    old.iter().zip(new.iter()).map(|(oi, ni)| oi - step * ni).collect()
}


pub fn gd<T: api::RecordMeta>(ds: &api::DataSet<T>, grad: &Gradient<T>, step: api::Number, e: api::Number) -> Parameters {
    if ds.target_count() != 2 {
        panic!("Not a binary target!");
    }
    let mut params = vec![0.0; vectorize::vector_len(ds) + 1];
    loop {
        let next = next_parameters(&params, &grad(ds), step);
        if distance(&next, &params) < e {
            break;
        } else {
            params = next;
        }
    };
    params
}
