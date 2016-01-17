use std::f64;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Category {
	A,
	B,
	C
}

#[derive(Copy, Clone, Debug)]
enum Class {
	C1,
	C2
}

#[derive(Copy, Clone, Debug)]
struct Record {
	category: Category,
    number: u8,	
    boolean: bool,
    class: Class
}

fn e_class(data: &Vec<Record>) -> f64 {
	let total = data.len();
	let mut classes = vec![0u8, 0u8];
	for record in data {
		let i = match record.class {
			Class::C1 => 0,
			Class::C2 => 1,
		};
		classes[i] += 1;
	}
	if classes[0] == 0 || classes[1] == 0 {
		return 0f64
	}
	let mut result = 0f64;
	for class in classes {
		let p = (class as f64) / (total as f64);
		result += p * p.log(2f64)
	}
	-result
}

fn e_category_helper(data: &Vec<Record>, category: &Category) -> f64 {
	let mut total = 0;
	let mut classes = vec![0u8, 0u8];
	for record in data {
		if record.category == *category {
			let i = match record.class {
				Class::C1 => 0,
				Class::C2 => 1,
			};
			classes[i] += 1;
			total += 1;
		}
	}
	let mut result = 0f64;
	for class in classes {
		if class != 0 {	
			let p = (class as f64) / (total as f64);
			result += p * p.log(2f64)
		} 
	}
	-result
	
}

fn e_category(data: &Vec<Record>) -> f64 {
	let total = data.len();
	let mut ccs = vec![0u8, 0u8, 0u8];
	let categories = vec![Category::A, Category::B, Category::C];
	for record in data {
		let i = match record.category {
				Category::A => 0,
				Category::B => 1,
				Category::C => 2,
		};
		ccs[i] += 1;
	}
	let mut result = 0f64;
	for i in 0..3 {
		let p = (ccs[i] as f64) / (total as f64);
		result += p * e_category_helper(data, &categories[i]); 
	}
	result
}

fn e_boolean_helper(data: &Vec<Record>, boolean: bool) -> f64 {
	let mut total = 0;
	let mut classes = vec![0u8, 0u8];
	for record in data {
		if record.boolean == boolean {
			let i = match record.class {
				Class::C1 => 0,
				Class::C2 => 1,
			};
			classes[i] += 1;
			total += 1;
		}
	}
	let mut result = 0f64;
	for class in classes {
		if class != 0 {	
			let p = (class as f64) / (total as f64);
			result += p * p.log(2f64)
		} 
	}
	-result
	
}

fn e_boolean(data: &Vec<Record>) -> f64 {
	let total = data.len();
	let mut ccs = vec![0u8, 0u8];
	let values = vec![false, true];
	for record in data {
		let i = match record.boolean {
				false => 0,
				true => 1,
		};
		ccs[i] += 1;
	}
	let mut result = 0f64;
	for i in 0..2 {
		let p = (ccs[i] as f64) / (total as f64);
		result += p * e_boolean_helper(data, values[i]); 
	}
	result
}

fn e_number_helper(data: &Vec<Record>, sign: bool, bound: u8) -> f64 {
	let mut total = 0;
	let mut classes = vec![0u8, 0u8];
	for record in data {
		if (!sign && record.number <= bound) || (sign && record.number > bound) {
			let i = match record.class {
				Class::C1 => 0,
				Class::C2 => 1,
			};
			classes[i] += 1;
			total += 1;
		}
	}
	let mut result = 0f64;
	for class in classes {
		if class != 0 {	
			let p = (class as f64) / (total as f64);
			result += p * p.log(2f64)
		} 
	}
	-result
	
}

fn e_number(data: &Vec<Record>, bound: u8) -> f64 {
	let total = data.len();
	let mut ccs = vec![0u8, 0u8];
	let bounds = vec![false, true];
	for record in data {
		let i = if record.number <= bound {
			0
		} else {
			1
		};
		ccs[i] += 1;
	}
	let mut result = 0f64;
	for i in 0..2 {
		let p = (ccs[i] as f64) / (total as f64);
		result += p * e_number_helper(data, bounds[i], bound); 
	}
	result
}

fn best_bound(data: &Vec<Record>) -> u8 {
	let mut bounds = data.iter().map(|ref record| record.number).collect::<Vec<u8>>();
	bounds.sort();
	bounds.dedup();
	let len = bounds.len();
	bounds.remove(len-1);
	let mut min_e = f64::MAX;
	let mut min_bound = 0u8;
	for bound in bounds {
		let current_e = e_number(data, bound);
		if current_e < min_e {
			min_bound = bound;
			min_e = current_e;
		}
	}
	min_bound
}

fn id3(data: &Vec<Record>, category: Option<Category>, number: Option<(char, u8)>, boolean: Option<bool>) {
	if data.len() == 0 {
		return
	}
	let e = e_class(data);
	let bound = best_bound(data);
	let category_gain = match category {
		Some(_) => 0f64,
		None => e - e_category(data)
	};
	let number_gain = match number {
		Some(_) => 0f64,
		None => e - e_number(data, bound)
	};
	let boolean_gain = match boolean {
		Some(_) => 0f64,
		None => e - e_boolean(data)
	};
	println!("Node[Category={:?}, Number={:?}, Boolean={:?}]", category, number, boolean);
	if e == 0f64 {
		return
	}
	if category_gain > number_gain {
		if category_gain > boolean_gain {
			let category_a: Vec<Record> = data.iter().filter(|ref record| record.category == Category::A).map(|ref record| **record).collect();
			id3(&category_a, Option::Some(Category::A), number, boolean);
			let category_b: Vec<Record> = data.iter().filter(|ref record| record.category == Category::B).map(|ref record| **record).collect();
			id3(&category_b, Option::Some(Category::B), number, boolean);
			let category_c: Vec<Record> = data.iter().filter(|ref record| record.category == Category::C).map(|ref record| **record).collect();
			id3(&category_c, Option::Some(Category::C), number, boolean);
		} else {
			let category_true: Vec<Record> = data.iter().filter(|ref record| record.boolean).map(|ref record| **record).collect();
			id3(&category_true, category, number, Option::Some(true));
			let category_false: Vec<Record> = data.iter().filter(|ref record| !record.boolean).map(|ref record| **record).collect();
			id3(&category_false, category, number, Option::Some(false));
    	} 
	} else {
		if number_gain > boolean_gain {
			let category_more: Vec<Record> = data.iter().filter(|ref record| record.number > bound).map(|ref record| **record).collect();
			id3(&category_more, category, Option::Some(('>', bound)), boolean);
			let category_less: Vec<Record> = data.iter().filter(|ref record| record.number <= bound).map(|ref record| **record).collect();
			id3(&category_less, category, Option::Some(('<', bound)), boolean);
		} else {
			let category_true: Vec<Record> = data.iter().filter(|ref record| record.boolean).map(|ref record| **record).collect();
			id3(&category_true, category, number, Option::Some(true));
			let category_false: Vec<Record> = data.iter().filter(|ref record| !record.boolean).map(|ref record| **record).collect();
			id3(&category_false, category, number, Option::Some(false));
		}
	}
}

fn main() {
    println!("Hello, ID3!");
    let data = vec![
		Record{category: Category::A, number: 70, boolean: true, class: Class::C1},
		Record{category: Category::A, number: 90, boolean: true, class: Class::C2},
		Record{category: Category::A, number: 85, boolean: false, class: Class::C2},
		Record{category: Category::A, number: 95, boolean: false, class: Class::C2},
		Record{category: Category::A, number: 70, boolean: false, class: Class::C1},
		Record{category: Category::B, number: 90, boolean: true, class: Class::C1},
		Record{category: Category::B, number: 78, boolean: false, class: Class::C1},
		Record{category: Category::B, number: 65, boolean: true, class: Class::C1},
		Record{category: Category::B, number: 75, boolean: false, class: Class::C1},
		Record{category: Category::C, number: 80, boolean: true, class: Class::C2},
		Record{category: Category::C, number: 70, boolean: true, class: Class::C2},
		Record{category: Category::C, number: 80, boolean: false, class: Class::C1},
		Record{category: Category::C, number: 80, boolean: false, class: Class::C1},
		Record{category: Category::C, number: 96, boolean: false, class: Class::C1}
	];
	println!("{}", e_class(&data));
	println!("{}", e_category(&data));
	println!("{}", e_boolean(&data));
	let bound = best_bound(&data);
	println!("{}", bound);
	println!("{}", e_number(&data, bound));
	id3(&data, Option::None, Option::None, Option::None);
}
