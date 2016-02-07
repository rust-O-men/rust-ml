use super::super::api;

use std::collections::HashSet;
use std::mem;

pub type Selector<T> = Fn(&api::DataSet<T>, api::Feature, usize) -> Vec<api::Number>;

fn number_bound<T: api::RecordMeta>(record: &T, feature: api::Feature, bound: api::Number) -> bool {
    match record.number_value(feature) {
        Some(value) => value > bound,
        None => false
    }
}

pub fn bounds_criterions<T: api::RecordMeta>(data: &api::DataSet<T>, selector: &Selector<T>, n: usize) -> Vec<Box<api::Criterion<T>>> {
    let mut result: Vec<Box<api::Criterion<T>>> = Vec::new();
    for f in 0..data.records[0].0.feature_count() {
        if data.records[0].0.feature_type(f) == api::FeatureType::Number {
            let bounds = selector(data, f, n);
            for bound in bounds {
                result.push(Box::new(move |rec: &T| number_bound(rec, f, bound)));
                result.push(Box::new(move |rec: &T| !number_bound(rec, f, bound)));
            }
        }
    }
    result
}

pub fn uniform_selector<T: api::RecordMeta>(data: &api::DataSet<T>, feature: api::Feature, n: usize) -> Vec<api::Number> {
    let mut set: HashSet<u64> = HashSet::new();
    let mut result = Vec::new();
    let mut i = 0;
    for (j, record) in data.records.iter().enumerate() {
        if i != j % n {
            match record.0.number_value(feature) {
                Some(v) if !v.is_nan() => set.insert(unsafe{mem::transmute(v)}),
                _ => false 
            };
            i = j % n;
        } 
    }
    for packed in set.iter() {
        result.push(unsafe{mem::transmute(*packed)});
    }
    result
}

