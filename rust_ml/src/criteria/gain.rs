use super::super::api;

pub fn information_gain<T: api::RecordMeta, F>(data: &api::DataSet<T>, criterion: &F) -> f64 where F: Fn(&T) -> bool {
    let e = target_entropy(data);
    let fe = entropy(data, criterion);
    e - fe
}

fn entropy_helper<T: api::RecordMeta, F>(data: &api::DataSet<T>, value: bool, criterion: &F) -> f64 where F: Fn(&T) -> bool {
    let mut total = 0;
    let mut classes = vec![0; data.target_count];
    for record in data.records.iter() {
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

fn entropy<T: api::RecordMeta, F>(data: &api::DataSet<T>, criterion: &F) -> f64 where F: Fn(&T) -> bool {
    let total = data.records.len();
    let mut ccs = vec![0u8, 0u8];
    let values = vec![false, true];
    for record in data.records.iter() {
	let i = match criterion(&record.0) {
	    false => 0,
	    true => 1,
	};
	ccs[i] += 1;
    }
    let mut result = 0f64;
    for i in 0..2 {
	let p = (ccs[i] as f64) / (total as f64);
	result += p * entropy_helper(data, values[i], criterion); 
    }
    result
}

fn target_entropy<T: api::RecordMeta>(data: &api::DataSet<T>) -> f64 {
    let total = data.records.len();
    let mut classes = vec![0; data.target_count];
    for record in data.records.iter() {
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
