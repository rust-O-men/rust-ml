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


