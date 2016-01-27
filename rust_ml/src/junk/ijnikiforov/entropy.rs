
mod api {

	pub type Target = u32;
	pub type Number = f64;
	pub type Category = u32;

	pub type DataSet<T> = Vec<(T, Target)>;

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

	pub trait DataSetMeta {

		fn target_count(&self) -> usize;
	    
	}

}

mod data {

	use ::api;

	pub struct Record {
		pub category: api::Category,
    	pub number: api::Number,	
    	pub boolean: bool
	}

	impl api::RecordMeta for Record {

		fn feature_count(&self) -> usize {
			3
		}

		fn feature_name(&self, feature: usize) -> String {
			match feature {
			    0 => "category".to_string(),
			    1 => "number".to_string(),
			    2 => "boolean".to_string(),
			    _ => panic!("Unknown feature")
			}
		}

		fn feature_type(&self, feature: usize) -> api::FeatureType {
			match feature {
			    0 => api::FeatureType::Category,
			    1 => api::FeatureType::Number,
			    2 => api::FeatureType::Boolean,
			    _ => panic!("Unknown feature")
			}	
		}

		fn category_count(&self, feature: usize) -> usize {
			match feature {
			    0 => 3,
			    _ => panic!("Unknown feature")
			}	

		}

	}

	impl api::DataSetMeta for api::DataSet<Record> {

		fn target_count(&self) -> usize {
			2
		}

	}

	pub fn read_data() -> api::DataSet<Record> {
		let result = vec![
			(Record{category: 0, number: 70.0, boolean: true }, 0),
			(Record{category: 0, number: 90.0, boolean: true }, 1),
			(Record{category: 0, number: 85.0, boolean: false}, 1),
			(Record{category: 0, number: 95.0, boolean: false}, 1),
			(Record{category: 0, number: 70.0, boolean: false}, 0),
			(Record{category: 1, number: 90.0, boolean: true }, 0),
			(Record{category: 1, number: 78.0, boolean: false}, 0),
			(Record{category: 1, number: 65.0, boolean: true }, 0),
			(Record{category: 1, number: 75.0, boolean: false}, 0),
			(Record{category: 2, number: 80.0, boolean: true }, 1),
			(Record{category: 2, number: 70.0, boolean: true }, 1),
			(Record{category: 2, number: 80.0, boolean: false}, 0),
			(Record{category: 2, number: 80.0, boolean: false}, 0),
			(Record{category: 2, number: 96.0, boolean: false}, 0)
		];
		result
	}


}

mod solve_func {

	use ::api;

	pub fn information_gain<T: api::RecordMeta, F>(data: &api::DataSet<T>, criterion: &F, target_count: usize) -> f64 where F: Fn(&T) -> bool {
		let e = target_entropy(data, target_count);
		let fe = entropy(data, criterion, target_count);
		e - fe
	}

	fn entropy_helper<T: api::RecordMeta, F>(data: &api::DataSet<T>, value: bool, criterion: &F, target_count: usize) -> f64 where F: Fn(&T) -> bool {
		let mut total = 0;
		let mut classes = vec![0; target_count];
		for record in data.iter() {
			if criterion(&record.0) == value {
				classes[record.1 as usize] += 1;
				total += 1;
			}
		}
		let mut result = 0f64;
		for class in classes.iter() {
			if *class != 0 {	
				let p = (*class as f64) / (total as f64);
				result += p * p.log2()
			} 
		}
		-result
	}

	fn entropy<T: api::RecordMeta, F>(data: &api::DataSet<T>, criterion: &F, target_count: usize) -> f64 where F: Fn(&T) -> bool {
		let total = data.len();
		let mut ccs = vec![0u8, 0u8];
		let values = vec![false, true];
		for record in data.iter() {
			let i = match criterion(&record.0) {
					false => 0,
					true => 1,
			};
			ccs[i] += 1;
		}
		let mut result = 0f64;
		for i in 0..2 {
			let p = (ccs[i] as f64) / (total as f64);
			result += p * entropy_helper(data, values[i], criterion, target_count); 
		}
		result
	}

	fn target_entropy<T: api::RecordMeta>(data: &api::DataSet<T>, target_count: usize) -> f64 {
		let total = data.len();
		let mut classes = vec![0; target_count];
		for record in data.iter() {
			classes[record.1 as usize] += 1;
		}
		for class in classes.iter() {
			if *class == 0 {
				return 0f64	
			}
		}
		let mut result = 0f64;
		for class in classes.iter() {
			let p = (*class as f64) / (total as f64);
			result += p * p.log2()
		}
		-result
	} 

}

fn main() {
	use api::DataSetMeta;

    println!("Hello, ID3!");
    let data = data::read_data(); 
    let entropy = solve_func::information_gain(&data, &|rec| rec.boolean, data.target_count());  
    println!("{}", entropy); 
}
