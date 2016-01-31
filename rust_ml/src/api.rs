pub type Target = u32;
pub type Number = f64;
pub type Category = u32;

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

	fn feature_name(&self, feature: usize) -> String;
	fn feature_type(&self, feature: usize) -> FeatureType;

	fn category_count(&self, feature: usize) -> usize;

}

pub type Criterion<T> = Fn(&T) -> bool;

pub type Solver<T> = Fn(&DataSet<T>, &DataSetView, &Criterion<T>) -> f64;

pub type DataSetView = Vec<usize>;

pub fn all_view<T>(data: &DataSet<T>) -> DataSetView {
	(0usize..data.records.len()).collect()
}
