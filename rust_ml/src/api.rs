use std::collections::HashSet;


pub type Target = u32;
pub type Number = f64;
pub type Category = u32;
pub type Feature = usize;


#[derive(PartialEq, Eq, Clone)]
pub enum FeatureType {
    Boolean,
    Category,
    Number
}


pub trait RecordMeta: Clone {
        
    fn feature_count(&self) -> usize;

    fn feature_name(&self, feature: Feature) -> String;
    fn feature_type(&self, feature: Feature) -> FeatureType;

    fn category_count(&self, feature: Feature) -> usize;

    fn number_value(&self, feature: Feature) -> Option<Number>;
    fn category_value(&self, feature: Feature) -> Option<Category>;
    fn bool_value(&self, feature: Feature) -> Option<bool>;
}


pub struct DataSetMeta {
    pub target_count: usize,
    pub features: Vec<FeatureType>
}


pub struct DataSet<T: RecordMeta> {
    records: Vec<(T, Target)>,
    meta: DataSetMeta,
}


pub struct DataSetIterator<'a, T: 'a> {
    records: &'a Vec<(T, Target)>,
    current: usize,
    max: usize,
    view: Option<&'a DataSetView>
} 


pub struct DataSetView {
    indexes: HashSet<usize>
}


impl DataSetView {

    pub fn empty() -> Self {
        DataSetView{indexes: HashSet::new()}
    }

    pub fn full(max_index: usize) -> Self {
        let mut is = HashSet::new();
        for index in 0..max_index-1 {
            is.insert(index);
        }
        DataSetView{indexes: is}
    }

    pub fn new(indexes: Vec<usize>) -> Self {
        let mut is = HashSet::new();
        for index in indexes {
            is.insert(index);
        }
        DataSetView{indexes: is}
    }

    pub fn add_index(&mut self, index: usize) {
        self.indexes.insert(index);
    }

    pub fn remove_index(&mut self, index: usize) {
        self.indexes.remove(&index);
    }

    pub fn is_empty(&self) -> bool {
        self.indexes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.indexes.len()
    }

}


impl<T: RecordMeta> DataSet<T> {

    pub fn new(meta: DataSetMeta, records: Vec<(T, Target)>) -> Self {
        // TODO: here we should check that all records has the same meta
        DataSet{records: records, meta: meta}
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }        

    pub fn target_count(&self) -> usize {
        self.meta.target_count
    }

    pub fn feature_count(&self) -> usize {
        self.meta.features.len()
    }

    pub fn feature_type(&self, feature: Feature) -> FeatureType {
        self.meta.features[feature].clone()
    }    

    pub fn records(&self) -> DataSetIterator<T> {
        DataSetIterator{records: &self.records, current: 0, max: self.records.len(), view: None}
    }

    pub fn view_records<'a>(&'a self, view: &'a DataSetView) -> DataSetIterator<T> {
        DataSetIterator{records: &self.records, current: 0, max: self.records.len(), view: Some(view)}    
    }

    pub fn record_clone(&self, index: usize) -> T {
        self.records[index].0.clone()
    }

}


impl<'a, T: RecordMeta> Iterator for DataSetIterator<'a, T> {

    type Item = &'a (T, Target);

    fn next(&mut self) -> Option<&'a (T, Target)> {
        if self.current < self.max {
            match self.view {
                Some(ref view) => {
                    while view.indexes.contains(&self.current) {
                        self.current = self.current + 1;    
                    };
                }
                None => {}
            }
            let result = Some(&self.records[self.current]);
            self.current = self.current + 1;
            return result
        } else {
            return None
        }
    }

}


pub type Criterion<T> = Fn(&T) -> bool;

pub type Solver<T> = Fn(&DataSet<T>, &DataSetView, &Criterion<T>) -> f64;

