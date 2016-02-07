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

    fn number_value(&self, feature: usize) -> api::Number {
        match feature {
            1 => self.number,
            _ => panic!("Unknown feature")
        }
    }

    fn category_value(&self, feature: usize) -> api::Category {
        match feature {
            0 => self.category,
            _ => panic!("Unknown feature")
        }

    }

    fn bool_value(&self, feature: usize) -> bool {
        match feature {
            2 => self.boolean,
            _ => panic!("Unknown feature")
        }

    }

}

pub const TARGET_1: api::Target = 0;
pub const TARGET_2: api::Target = 1;

pub const A_CATEGORY: api::Category = 0;
pub const B_CATEGORY: api::Category = 1;
pub const C_CATEGORY: api::Category = 2;

pub fn read_data() -> api::DataSet<Record> {
    let result = api::DataSet{
        records: vec![
            (Record{category: A_CATEGORY, number: 70.0, boolean: true }, TARGET_1),
            (Record{category: A_CATEGORY, number: 90.0, boolean: true }, TARGET_2),
            (Record{category: A_CATEGORY, number: 85.0, boolean: false}, TARGET_2),
            (Record{category: A_CATEGORY, number: 95.0, boolean: false}, TARGET_2),
            (Record{category: A_CATEGORY, number: 70.0, boolean: false}, TARGET_1),
            (Record{category: B_CATEGORY, number: 90.0, boolean: true }, TARGET_1),
            (Record{category: B_CATEGORY, number: 78.0, boolean: false}, TARGET_1),
            (Record{category: B_CATEGORY, number: 65.0, boolean: true }, TARGET_1),
            (Record{category: B_CATEGORY, number: 75.0, boolean: false}, TARGET_1),
            (Record{category: C_CATEGORY, number: 80.0, boolean: true }, TARGET_2),
            (Record{category: C_CATEGORY, number: 70.0, boolean: true }, TARGET_2),
            (Record{category: C_CATEGORY, number: 80.0, boolean: false}, TARGET_1),
            (Record{category: C_CATEGORY, number: 80.0, boolean: false}, TARGET_1),
            (Record{category: C_CATEGORY, number: 96.0, boolean: false}, TARGET_1)
        ],
        target_count: 2
    };
    result
}
