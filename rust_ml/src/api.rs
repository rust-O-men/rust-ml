pub type Target = u32;
pub type Number = f64;
pub type Category = u32;
pub type Feature = usize;

pub struct DataSet<T> {
    pub records: Vec<(T, Target)>,
    pub target_count: usize
}

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

    fn number_value(&self, feature: Feature) -> Number;
    fn category_value(&self, feature: Feature) -> Category;
    fn bool_value(&self, feature: Feature) -> bool;
}

pub type Criterion<T> = Fn(&T) -> bool;

pub type Solver<T> = Fn(&DataSet<T>, &DataSetView, &Criterion<T>) -> f64;

pub type DataSetView = Vec<usize>;

pub fn all_view<T>(data: &DataSet<T>) -> DataSetView {
    (0usize..data.records.len()).collect()
}
