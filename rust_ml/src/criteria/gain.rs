use super::super::api;

pub fn information_gain<T: api::RecordMeta>(data: &api::DataSet<T>, criterion: &api::Criterion<T>) -> f64 {
	let e = target_entropy(data);
	let fe = entropy(data, criterion);
	e - fe
}

fn entropy_helper<T: api::RecordMeta>(data: &api::DataSet<T>) -> f64 {
    let total = data.total_count() as f64;
    let classes = data.class_count();
    
    let mut result = 0f64;
    for (_, v) in classes.iter() {
	let p = (*v as f64) / total;
	result += p * p.log2()
    }
    -result
}

fn entropy<T: api::RecordMeta>(data: &api::DataSet<T>, criterion: &api::Criterion<T>) -> f64 {
    let total = data.total_count() as f64;
    let (ls,rs) = data.split(criterion);
    (ls.total_count() as f64)/total * entropy_helper(&ls)
        + (rs.total_count() as f64)/total * entropy_helper(&rs)
}

fn target_entropy<T: api::RecordMeta>(data: &api::DataSet<T>) -> f64 {
    let classes = data.class_count(); 
    let total = data.total_count() as f64;
    let mut result = 0f64;
    for (_, v) in classes {
        let p = (v as f64)/total;
        result += p * p.log2();
    }
    -result
}
