use super::super::api;
use super::super::features::vectorize;

pub type Parameters = Vec<api::Number>;

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

fn q<T: api::RecordMeta>(ds: &api::DataSet<T>, params: &Parameters, z: &Fn(api::Number) -> api::Number) -> api::Number {
    ds.records.iter().fold(0.0, |acc, ref rec| acc + z(f(&rec.0, params) * normalize_target(rec.1)))
}

fn q_partial_derivative<T: api::RecordMeta>(ds: &api::DataSet<T>, params: &Parameters, z: &Fn(api::Number) -> api::Number, i: usize, h: api::Number) -> api::Number {
    let next: Parameters = params.iter().enumerate().map(|(n, &x)| x + if i == n {h} else {0.0}).collect();
    (q(ds, &next, z) - q(ds, params, z)) / h
}

fn q_gradient<T: api::RecordMeta>(ds: &api::DataSet<T>, params: &Parameters, z: &Fn(api::Number) -> api::Number, h: api::Number) -> Parameters {
    let mut result = Vec::new();
    for i in 0..params.len() {
        result.push(q_partial_derivative(ds, params, z, i, h));
    } 
    result
}

pub fn gd<T: api::RecordMeta>(ds: &api::DataSet<T>, z: &Fn(api::Number) -> api::Number, e: f64) -> Parameters {
    if ds.target_count != 2 {
        panic!("Only binary classification supported!");
    }
    let mut params = vec![0.0; ds.records[0].0.feature_count() + 1];
    let mut value = q(ds, &params, z);
    loop {
        let gradient = q_gradient(ds, &params, z, e);
        for (i, g) in gradient.iter().enumerate() {
            params[i] -= 0.01 * g;
        }
        let next = q(ds, &params, z);
        if (value - next) < e {
            return params
        } else {
            value = next;
        }
    }
}
