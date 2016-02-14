
mod api {

    use std::collections::HashSet;

    pub type Target = u32;

    pub enum FeatureType {
        Boolean,
        Category,
        Number
    }

    pub trait RecordMeta {

        fn some_record_meta(&self) -> bool;

    }

    pub struct DataSetMeta {
        pub target_count: usize,
        pub features: Vec<(usize, FeatureType)>
    }

    pub struct DataSet<T: RecordMeta> {
        records: Vec<(T, Target)>,
        meta: DataSetMeta,
    }

    pub struct DataSetIterator<'a, T: 'a> {
        records: &'a Vec<(T, Target)>,
        current: usize,
        max: usize,
        view: Option<&'a DataSetView>
    } 

    pub struct DataSetView {
        indexes: HashSet<usize> // bitset should be here
    }

    impl DataSetView {

        pub fn empty() -> Self {
            DataSetView{indexes: HashSet::new()}
        }

        pub fn new(indexes: Vec<usize>) -> Self {
            let mut is = HashSet::new();
            for index in indexes {
                is.insert(index);
            }
            DataSetView{indexes: is}
        }

        pub fn add_index(&mut self, index: usize) {
            self.indexes.insert(index);
        }

    }

    impl<T: RecordMeta> DataSet<T> {

        pub fn new(meta: DataSetMeta, records: Vec<(T, Target)>) -> Self {
            // here we should check that all records has the same meta
            DataSet{records: records, meta: meta}
        }

        pub fn is_empty(&self) -> bool {
            self.records.is_empty()
        }

        pub fn len(&self) -> usize {
            self.records.len()
        }        

        pub fn target_count(&self) -> usize {
            self.meta.target_count
        }

        pub fn records(&self) -> DataSetIterator<T> {
            DataSetIterator{records: &self.records, current: 0, max: self.records.len(), view: None}
        }

        pub fn view_records<'a>(&'a self, view: &'a DataSetView) -> DataSetIterator<T> {
            DataSetIterator{records: &self.records, current: 0, max: self.records.len(), view: Some(view)}    
        }

    }

    impl<'a, T: RecordMeta> Iterator for DataSetIterator<'a, T> {

        type Item = &'a (T, Target);

        fn next(&mut self) -> Option<&'a (T, Target)> {
            if self.current < self.max {
                match self.view {
                    Some(ref view) => {
                        while view.indexes.contains(&self.current) {
                            self.current = self.current + 1;    
                        };
                    }
                    Node => {}
                }
                let result = Some(&self.records[self.current]);
                self.current = self.current + 1;
                return result
            } else {
                return None
            }
        }

    }

} 

mod data {

    use ::api;

    #[derive(Debug)]
    pub struct Record {
        pub zopa: u32
    }

    impl api::RecordMeta for Record {

        fn some_record_meta(&self) -> bool {
            true
        }

    }

    pub fn get_data() -> api::DataSet<Record> { 
        api::DataSet::new(
            api::DataSetMeta {
                target_count: 2,
                features: vec![
                    (0, api::FeatureType::Number),
                    (1, api::FeatureType::Boolean)
                ]
            },
            vec![
                (Record{zopa: 0}, 0),
                (Record{zopa: 1}, 1)
            ]
        )
    }


}

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
    use data;
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
