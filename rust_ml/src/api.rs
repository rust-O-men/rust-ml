pub type Target = u32;
pub type Number = f64;
pub type Category = u32;
pub type Feature = usize;

pub struct DataSet<T: RecordMeta> {
    pub records: Vec<(T, Target)>,
    pub target_count: usize
}

#[derive(PartialEq, Eq)]
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

pub type Solver<T> = Fn(&DataSet<T>, &DataSetView, &Criterion<T>) -> f64;

pub type DataSetView = Vec<usize>;

pub fn all_view<T: RecordMeta>(data: &DataSet<T>) -> DataSetView {
    (0usize..data.records.len()).collect()
}
