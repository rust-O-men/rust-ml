use api;
use data;

fn test<T: api::RecordMeta>(ds: &api::DataSet<T>) {
    println!("{}", ds.target_count());
    for record in ds.records() {
        println!("{}", record.0.some_record_meta());
    }    
}

fn test2<T: api::RecordMeta>(ds: &api::DataSet<T>, view: &mut api::DataSetView, index: usize) {
    if ds.len() == index {
        return
    }
    for record in ds.view_records(view) {
        println!("{}: {}", index, record.0.some_record_meta());
    }
    view.add_index(index);
    test2(ds, view, index + 1)    
}

fn main() {
    println!("Hello, API!");
    let ds = data::get_data();
    println!("is_empty = {}, target_count = {}", ds.is_empty(), ds.target_count());
    for record in ds.records() {
        println!("{:?}", record);
    }
    for record in ds.view_records(&api::DataSetView::new(vec![0])) {
        println!("{:?}", record);
    }
    test(&ds);
    test2(&ds, &mut api::DataSetView::empty(), 0);
}

