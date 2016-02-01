use super::super::api;

pub struct Record {
    pub category: api::Category,
    pub number: api::Number,	
    pub boolean: bool
}

impl api::RecordMeta for Record {

    fn feature_count(&self) -> usize {
	3
    }

    fn feature_name(&self, feature: usize) -> String {
	match feature {
	    0 => "category".to_string(),
	    1 => "number".to_string(),
	    2 => "boolean".to_string(),
	    _ => panic!("Unknown feature")
	}
    }

    fn feature_type(&self, feature: usize) -> api::FeatureType {
	match feature {
	    0 => api::FeatureType::Category,
	    1 => api::FeatureType::Number,
	    2 => api::FeatureType::Boolean,
	    _ => panic!("Unknown feature")
	}	
    }

    fn category_count(&self, feature: usize) -> usize {
	match feature {
	    0 => 3,
	    _ => panic!("Unknown feature")
	}

    }

}

pub fn read_data() -> api::DataSet<Record> {
    let result = api::DataSet{
	records: vec![
	    (Record{category: 0, number: 70.0, boolean: true }, 0),
	    (Record{category: 0, number: 90.0, boolean: true }, 1),
	    (Record{category: 0, number: 85.0, boolean: false}, 1),
	    (Record{category: 0, number: 95.0, boolean: false}, 1),
	    (Record{category: 0, number: 70.0, boolean: false}, 0),
	    (Record{category: 1, number: 90.0, boolean: true }, 0),
	    (Record{category: 1, number: 78.0, boolean: false}, 0),
	    (Record{category: 1, number: 65.0, boolean: true }, 0),
	    (Record{category: 1, number: 75.0, boolean: false}, 0),
	    (Record{category: 2, number: 80.0, boolean: true }, 1),
	    (Record{category: 2, number: 70.0, boolean: true }, 1),
	    (Record{category: 2, number: 80.0, boolean: false}, 0),
	    (Record{category: 2, number: 80.0, boolean: false}, 0),
	    (Record{category: 2, number: 96.0, boolean: false}, 0)
		],
	target_count: 2
    };
    result
}
