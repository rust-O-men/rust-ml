use super::super::api;

pub struct Tree {
    root: Box<Node>
}

enum Node {
    Fork(Box<Node>, Box<Node>, usize),
    Target(api::Target)
 }

pub fn id3<T: api::RecordMeta>(data: &api::DataSet<T>, solver: &api::Solver<T>, criterions: &Vec<Box<api::Criterion<T>>>) -> Tree {
    Tree {
        root: create_node(data, &api::DataSetView::full(data.len()), solver, criterions)
    }
}

fn create_node<T: api::RecordMeta>(data: &api::DataSet<T>, view: &api::DataSetView, solver: &api::Solver<T>, criterions: &Vec<Box<api::Criterion<T>>>) -> Box<Node> {
    let mut max = 0f64;
    let mut index = 0usize;
    for (i, criterion) in criterions.iter().enumerate() {
        let current = solver(data, view, &**criterion);
        if max < current {
            max = current;
            index = i;
        }
    }
    let criterion = &criterions[index];
    let mut left_records = api::DataSetView::empty();
    let mut right_records = api::DataSetView::empty();
    for (i, record) in data.view_records(view).enumerate() {
        if criterion(&record.0) {
            right_records.add_index(i)
        } else {
            left_records.add_index(i)
        }
    }
    if right_records.is_empty() || left_records.is_empty() {
        let target = if right_records.len() < left_records.len() {
            calculate_target(data, &left_records)
        } else {
            calculate_target(data, &right_records)
        };
        Box::new(Node::Target(target))
    } else {
        Box::new(Node::Fork(create_node(data, &left_records, solver, criterions), create_node(data, &right_records, solver, criterions), index))
    }
}

fn calculate_target<T: api::RecordMeta>(data: &api::DataSet<T>, view: &api::DataSetView) -> api::Target {
    let mut classes = vec![0; data.target_count()];
    for record in data.view_records(view) {
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

fn apply_node<T: api::RecordMeta>(node: &Box<Node>, record: &T, criterions: &Vec<Box<api::Criterion<T>>>) -> api::Target {
    match **node {
        Node::Target(target) => target,
        Node::Fork(ref left, ref right, criterion) => {
            if criterions[criterion](record) {
                apply_node(right, record, criterions)
            } else {
                apply_node(left, record, criterions)
            }
        }
    }
}

impl Tree {

    pub fn apply<T: api::RecordMeta>(&self, record: &T, criterions: &Vec<Box<api::Criterion<T>>>) -> api::Target {
        apply_node(&self.root, record, criterions)
    }
}
