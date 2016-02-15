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

