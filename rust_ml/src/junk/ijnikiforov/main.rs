
mod api {

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

	pub fn read_data() -> api::DataSet<Record> {
		let result = api::DataSet{
			records: vec![
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
			],
			target_count: 2
		};
		result
	}

}

mod solve_func {

	use ::api;

	pub fn information_gain<T: api::RecordMeta>(data: &api::DataSet<T>, view: &api::DataSetView, criterion: &api::Criterion<T>) -> f64 {
		let e = target_entropy(data, view);
		let fe = entropy(data, view, criterion);
		e - fe
	}

	fn entropy_helper<T: api::RecordMeta>(data: &api::DataSet<T>, view: &api::DataSetView, value: bool, criterion: &api::Criterion<T>) -> f64 {
		let mut total = 0;
		let mut classes = vec![0; data.target_count];
		for index in view.iter() {
			let record = &data.records[*index];
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

	fn entropy<T: api::RecordMeta>(data: &api::DataSet<T>, view: &api::DataSetView, criterion: &api::Criterion<T>) -> f64 {
		let total = data.records.len();
		let mut ccs = vec![0u8, 0u8];
		let values = vec![false, true];
		for index in view.iter() {
			let record = &data.records[*index];	
			let i = match criterion(&record.0) {
					false => 0,
					true => 1,
			};
			ccs[i] += 1;
		}
		let mut result = 0f64;
		for i in 0..2 {
			let p = (ccs[i] as f64) / (total as f64);
			result += p * entropy_helper(data, view, values[i], criterion); 
		}
		result
	}

	fn target_entropy<T: api::RecordMeta>(data: &api::DataSet<T>, view: &api::DataSetView) -> f64 {
		let total = data.records.len();
		let mut classes = vec![0; data.target_count];
		for index in view.iter() {
			let record = &data.records[*index];	
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

mod id3 {

	use api;

	pub struct Tree {
		root: Box<Node>
	}

	struct Node {
		left: Option<Box<Node>>,
		right: Option<Box<Node>>,
		criterion: Option<usize>,
		target: Option<api::Target>
	}

	pub fn id3<T>(data: &api::DataSet<T>, solver: &api::Solver<T>, criterions: &Vec<&api::Criterion<T>>) -> Tree {
		Tree {
			root: create_node(data, &api::all_view(data), solver, criterions)
		}
	}

	fn create_node<T>(data: &api::DataSet<T>, view: &api::DataSetView, solver: &api::Solver<T>, criterions: &Vec<&api::Criterion<T>>) -> Box<Node> {
		let mut max = 0f64;
		let mut index = 0usize;
		for (i, criterion) in criterions.iter().enumerate() {
			let current = solver(data, view, *criterion);
			if max < current {
				max = current;
				index = i;
			}
		}
		let criterion = criterions[index];
		let mut left_records = Vec::new();
		let mut right_records = Vec::new();
		for i in view.iter() {
			let record = &data.records[*i];
			if criterion(&record.0) {
				right_records.push(*i)
			} else {
				left_records.push(*i)
			}
		}
		if right_records.is_empty() || left_records.is_empty() {
			let target = if right_records.len() < left_records.len() {
				calculate_target(data, &left_records)
			} else {
				calculate_target(data, &right_records)
			};
			Box::new(Node{left: None, right: None, criterion: None, target: Some(target)})
		} else {
			Box::new(Node{
				left: Some(create_node(data, &left_records, solver, criterions)), 
				right: Some(create_node(data, &right_records, solver, criterions)), 
				criterion: Some(index), 
				target: None
			})
		}
	}

	fn calculate_target<T>(data: &api::DataSet<T>, view: &api::DataSetView) -> api::Target {
		let mut classes = vec![0; data.target_count];
		for index in view.iter() {
			let record = &data.records[*index];
			classes[record.1 as usize] += 1;
		}
		let mut max_class = 0;
		let mut target = 0;
		for (i, class) in classes.iter().enumerate() {
			if max_class < *class {
				max_class = *class;
				target = i;
			}
		}
		target as u32
	}

	fn apply_node<T>(node: &Box<Node>, record: &T, criterions: &Vec<&api::Criterion<T>>) -> api::Target {
		match node.target {
			Some(target) => target,
			None => {
				let criterion = node.criterion.unwrap();
				if criterions[criterion](record) {
					match node.right {
						Some(ref nnode) => apply_node(nnode, record, criterions),
						None => panic!("assimetric tree")
					}
				} else {
					match node.left {
						Some(ref nnode) => apply_node(nnode, record, criterions),
						None => panic!("assimetric tree")
					}
				}
			}
		}
	}

	impl Tree {

		pub fn apply<T>(&self, record: &T, criterions: &Vec<&api::Criterion<T>>) -> api::Target {
			apply_node(&self.root, record, criterions)
		}


	}

}

fn main() {
	println!("Hello, ID3!");
	let data = data::read_data(); 
	let entropy = solve_func::information_gain(&data, &api::all_view(&data), &|rec| rec.boolean);  
	println!("{}", entropy);
	let boolean = |rec: &data::Record| rec.boolean;
	let category_a = |rec: &data::Record| rec.category == 0;
	let category_b = |rec: &data::Record| rec.category == 1;
	let category_c = |rec: &data::Record| rec.category == 2;
	let number = |rec: &data::Record| rec.number > 80.0;
	let criterions :Vec<&api::Criterion<data::Record>> = vec![
		&boolean, 
		&category_a, 
		&category_b, 
		&category_c, 
		&number
	];
	let tree = id3::id3(&data, &solve_func::information_gain, &criterions);
	let target = tree.apply(&data.records[1].0, &criterions);
	println!("{}", target);
}
