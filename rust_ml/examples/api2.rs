
mod api {

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
        max: usize
    } 

    impl<T: RecordMeta> DataSet<T> {

        pub fn new(meta: DataSetMeta, records: Vec<(T, Target)>) -> DataSet<T> {
            // here we should check that all records has the same meta
            DataSet{records: records, meta: meta}
        }

        pub fn is_empty(&self) -> bool {
            self.records.is_empty()
        }

        pub fn target_count(&self) -> usize {
            self.meta.target_count
        }

        pub fn records(&self) -> DataSetIterator<T> {
            DataSetIterator{records: &self.records, current: 0, max: self.records.len()}
        }

    }

    impl<'a, T: RecordMeta> Iterator for DataSetIterator<'a, T> {

        type Item = &'a (T, Target);

        fn next(&mut self) -> Option<&'a (T, Target)> {
            if self.current < self.max {
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

fn main() {
    use data;
    println!("Hello, API!");
    let ds = data::get_data();
    println!("is_empty = {}, target_count = {}", ds.is_empty(), ds.target_count());
    for record in ds.records() {
        println!("{:?}", record);
    }
    test(&ds);
}