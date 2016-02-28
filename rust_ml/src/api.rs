pub type Target = u32;
pub type Number = f64;
pub type Category = u32;
pub type Feature = usize;

use std::rc::Rc;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::slice::Iter;

pub struct DataSet<T:RecordMeta>(Vec<Rc<(T, Target)>>, HashSet<DataSetMeta>);
#[derive(PartialEq, Eq, Hash)]
pub struct DataSetMeta(Target);

#[derive(PartialEq, Eq, Hash)]
pub enum FeatureType {
    Boolean,
    Category,
    Number
}

pub trait RecordMeta {
    fn feature_count(&self) -> usize;

    fn feature_name(&self, feature: Feature) -> String;
    fn feature_type(&self, feature: Feature) -> FeatureType;

    fn category_count(&self, feature: Feature) -> usize;

    fn number_value(&self, feature: Feature) -> Option<Number>;
    fn category_value(&self, feature: Feature) -> Option<Category>;
    fn bool_value(&self, feature: Feature) -> Option<bool>;
}

pub type Criterion<T> = Fn(&T) -> bool;

pub type Solver<T> = Fn(&DataSet<T>, &Criterion<T>) -> f64;

impl<T:RecordMeta> DataSet<T> {
    pub fn new() -> DataSet<T> {
        let v:Vec<Rc<(T, Target)>> = Vec::new();
        DataSet(v, HashSet::new())
    }

    pub fn add(&mut self, item:T, target:Target) {
        self.0.push(Rc::new((item, target)));
        self.1.insert(DataSetMeta::new(target));
    }

    pub fn add_tuple(&mut self, item:Rc<(T, Target)>) {
        self.0.push(item.clone());
        self.1.insert(DataSetMeta::new(item.1));
    }
    
    pub fn class_count(&self) -> HashMap<Target, usize> {
        let mut cs = HashMap::new();
        for r in &self.0 {
            let counter = cs.entry(r.1).or_insert(0);
            *counter += 1;
        }
        cs
    }

    pub fn iter(&self) -> Iter<Rc<(T, Target)>> {
        self.0.iter()
    }

    pub fn total_count(&self) -> usize {
        self.0.len()
    }

    pub fn split(&self, criterion: &Criterion<T>) -> (DataSet<T>, DataSet<T>) {
        let mut left = DataSet::new();
        let mut right = DataSet::new();
        for f in self.iter() {
            if criterion(&f.0) {
                left.add_tuple(f.clone());
            }
            else {
                right.add_tuple(f.clone())
            }
        }
        (left, right)
    }

    pub fn split_positive(&self, criterion: &Criterion<T>) -> DataSet<T> {
        self.iter().filter(|x| criterion(&x.0)).cloned().collect()
    }
}

impl DataSetMeta {
    pub fn new(t:Target) -> DataSetMeta{
        DataSetMeta(t)
    }
}

impl<T:RecordMeta> FromIterator<Rc<(T,Target)>> for DataSet<T> {
    fn from_iter<I: IntoIterator<Item=Rc<(T, Target)>>> (iterator:I) -> Self {
        let mut ds = DataSet::new();
        for i in iterator {
            ds.add_tuple(i.clone());
        }
        ds
    }
}

impl<T:RecordMeta> IntoIterator for DataSet<T> {
    type Item = Rc<(T, Target)>;
    type IntoIter = ::std::vec::IntoIter<Rc<(T,Target)>>;
    
    fn into_iter (self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

