use super::super::api;

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
