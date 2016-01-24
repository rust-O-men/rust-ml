

#[derive(PartialEq, Clone)]
pub enum FeatureType {
	Boolean,
	Category,
	Number
}

#[derive(Clone)]
pub struct MetaFeature {
	pub name: String,
	pub ftype: FeatureType,
	pub count: u32
}

pub type Target = u32;
pub type Number = f64;
pub type Category = u32;

pub trait Feature {
	fn is_missed(&self) -> bool;
	
	fn is_boolean(&self) -> bool;
	fn is_number(&self) -> bool;
	fn is_category(&self) -> bool;	

	fn as_boolean(&self) -> Option<bool>;
	fn as_number(&self) -> Option<Number>;
	fn as_category(&self) -> Option<Category>;
}

struct FeatureData<T> {
	value: Option<T>
}

impl FeatureData<bool> {
	
	fn new(v: bool) -> FeatureData<bool> {
		FeatureData{value: Some(v)}
	}

	fn missed() -> FeatureData<bool> {
		FeatureData{value: None}
	}
}

impl Feature for FeatureData<bool> {

	fn is_missed(&self) -> bool {
		!self.value.is_some()
	}

	fn is_boolean(&self) -> bool {
		true
	}

	fn is_number(&self) -> bool {
		false
	}

	fn is_category(&self) -> bool {
		false
	}

	fn as_boolean(&self) -> Option<bool> {
		self.value.clone()
	}

	fn as_number(&self) -> Option<Number> {
		None
	}

	fn as_category(&self) -> Option<Category> {
		None
	}

}

pub fn boolean(v: bool) -> Box<Feature> {
	Box::new(FeatureData::<bool>::new(v))
}

pub fn missed_boolean() -> Box<Feature> {
	Box::new(FeatureData::<bool>::missed())
}

impl FeatureData<Number> {
	
	fn new(v: Number) -> FeatureData<Number> {
		FeatureData{value: Some(v)}
	}

	fn missed() -> FeatureData<Number> {
		FeatureData{value: None}
	}
}

impl Feature for FeatureData<Number> {

	fn is_missed(&self) -> bool {
		!self.value.is_some()
	}

	fn is_boolean(&self) -> bool {
		false
	}

	fn is_number(&self) -> bool {
		true
	}

	fn is_category(&self) -> bool {
		false
	}

	fn as_boolean(&self) -> Option<bool> {
		None
	}

	fn as_number(&self) -> Option<Number> {
		self.value.clone()
	}

	fn as_category(&self) -> Option<Category> {
		None
	}

}

pub fn number(v: Number) -> Box<Feature> {
	Box::new(FeatureData::<Number>::new(v))
}

pub fn missed_number() -> Box<Feature> {
	Box::new(FeatureData::<Number>::missed())
}

impl FeatureData<Category> {
	
	fn new(v: Category) -> FeatureData<Category> {
		FeatureData{value: Some(v)}
	}

	fn missed() -> FeatureData<Category> {
		FeatureData{value: None}
	}
}

impl Feature for FeatureData<Category> {

	fn is_missed(&self) -> bool {
		!self.value.is_some()
	}

	fn is_boolean(&self) -> bool {
		false
	}

	fn is_number(&self) -> bool {
		false
	}

	fn is_category(&self) -> bool {
		true
	}

	fn as_boolean(&self) -> Option<bool> {
		None
	}

	fn as_number(&self) -> Option<Number> {
		None
	}

	fn as_category(&self) -> Option<Category> {
		self.value.clone()
	}

}

pub fn category(v: Category) -> Box<Feature> {
	Box::new(FeatureData::<Category>::new(v))
}

pub fn missed_category() -> Box<Feature> {
	Box::new(FeatureData::<Category>::missed())
}

pub struct Record {
	pub features: Vec<Box<Feature>>,
	pub target: Target
}

pub struct DataSet {
	meta: Vec<MetaFeature>,
	records: Vec<Record>,
	tc: u32
}

impl DataSet {

	fn new(meta_features: Vec<MetaFeature>, target_count: u32) -> DataSet {
		DataSet{records: Vec::new(), meta: meta_features, tc: target_count}
	}

	fn add(&mut self, record: Record) -> &mut DataSet {
		if self.meta.len() != record.features.len() {
			panic!("Invalid record type!")
		}
		if record.target >= self.tc {
			panic!("Invalid target!")	
		}
		for (i, f) in record.features.iter().enumerate() {
			let b = f.is_boolean() && self.meta[i].ftype == FeatureType::Boolean;
			let n = f.is_number() && self.meta[i].ftype == FeatureType::Number;
			let c = f.is_category() && self.meta[i].ftype == FeatureType::Category;
			if !b && !n && !c {
				panic!("Invalid data type!")
			}
		} 
		self.records.push(record);
		self
	}

}

pub fn target_entropy(data: &DataSet) -> f64 {
	let total = data.records.len();
	let mut classes = vec![0; data.tc as usize];
	for record in data.records.iter() {
		classes[record.target as usize] += 1;
	}
	for class in classes.iter() {
		if *class == 0 {
			return 0f64	
		}
	}
	let mut result = 0f64;
	for class in classes.iter() {
		let p = (*class as f64) / (total as f64);
		result += p * p.log(2f64)
	}
	-result
} 

fn boolean_entropy_helper(data: &DataSet, column: usize, boolean: bool) -> f64 {
	let mut total = 0;
	let mut classes = vec![0; data.tc as usize];
	for record in data.records.iter() {
		if record.features[column].as_boolean().unwrap() == boolean {
			classes[record.target as usize] += 1;
			total += 1;
		}
	}
	let mut result = 0f64;
	for class in classes.iter() {
		if *class != 0 {	
			let p = (*class as f64) / (total as f64);
			result += p * p.log(2f64)
		} 
	}
	-result
	
}

fn boolean_entropy(data: &DataSet, column: usize) -> f64 {
	if data.meta[column].ftype != FeatureType::Boolean {
		panic!("Not a boolean field!");
	}
	let total = data.records.len();
	let mut ccs = vec![0u8, 0u8];
	let values = vec![false, true];
	for record in data.records.iter() {
		let i = match record.features[column].as_boolean().unwrap() {
				false => 0,
				true => 1,
		};
		ccs[i] += 1;
	}
	let mut result = 0f64;
	for i in 0..2 {
		let p = (ccs[i] as f64) / (total as f64);
		result += p * boolean_entropy_helper(data, column, values[i]); 
	}
	result
}

pub fn field_entropy(data: &DataSet, name: &str) -> f64 {
	for (i, meta) in data.meta.iter().enumerate() {
		if meta.name == name {
			return match meta.ftype {
				FeatureType::Boolean => boolean_entropy(data, i as usize),
				_ => 0f64
			}
		}
	}
	panic!("Field not found!");
}

pub fn field_gain(data: &DataSet, name: &str) -> f64 {
	let e = target_entropy(data);
	let fe = field_entropy(data, name);
	e - fe
}

fn main() {
    println!("Hello, ID3!");
    let mut ds = DataSet::new(vec![
    	MetaFeature{name: "category".to_string(), ftype: FeatureType::Category, count: 3},
    	MetaFeature{name: "number".to_string(), ftype: FeatureType::Number, count: 0},
    	MetaFeature{name: "boolean".to_string(), ftype: FeatureType::Boolean, count: 2}
    ], 2);
    ds.add(
    	Record {
    		features: vec![category(0), number(70.0), boolean(true)], 
    		target: 0
    	}
    ).add(
    	Record {
    		features: vec![category(0), number(90.0), boolean(true)], 
    		target: 1
    	}
    ).add(
    	Record {
    		features: vec![category(0), number(85.0), boolean(false)], 
    		target: 1
    	}
    ).add(
    	Record {
    		features: vec![category(0), number(95.0), boolean(false)], 
    		target: 1
    	}
    ).add(
    	Record {
    		features: vec![category(0), number(70.0), boolean(false)], 
    		target: 0
    	}
    ).add(
    	Record {
    		features: vec![category(1), number(90.0), boolean(true)], 
    		target: 0
    	}
    ).add(
    	Record {
    		features: vec![category(1), number(78.0), boolean(false)], 
    		target: 0
    	}
    ).add(
    	Record {
    		features: vec![category(1), number(65.0), boolean(true)], 
    		target: 0
    	}
    ).add(
    	Record {
    		features: vec![category(1), number(75.0), boolean(false)], 
    		target: 0
    	}
    ).add(
    	Record {
    		features: vec![category(2), number(80.0), boolean(true)], 
    		target: 1
    	}
    ).add(
    	Record {
    		features: vec![category(2), number(70.0), boolean(true)], 
    		target: 1
    	}
    ).add(
    	Record {
    		features: vec![category(2), number(80.0), boolean(false)], 
    		target: 0
    	}
    ).add(
    	Record {
    		features: vec![category(2), number(80.0), boolean(false)], 
    		target: 0
    	}
    ).add(
    	Record {
    		features: vec![category(2), number(96.0), boolean(false)], 
    		target: 0
    	}
    );
    let te = target_entropy(&ds);
    println!("Target entropy={}", te);
    let be = field_entropy(&ds, "boolean");
    println!("Boolean entropy={}", be);
    let bg = field_gain(&ds, "boolean");
    println!("Boolean gain={}", bg);
    
}
