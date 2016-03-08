use super::super::api;

pub fn vector_len<T: api::RecordMeta>(ds: &api::DataSet<T>) -> usize {
    let mut result = 0;
    for f in 0..ds.feature_count() {
        match ds.feature_type(f) {
            api::FeatureType::Number => {
                result += 1;
            },
            api::FeatureType::Boolean => {
                result += 1;
            },
            api::FeatureType::Category => {
                for v in 0..ds.category_count(f) {
                    result += 1;
                };  
            }
        };
    }    
    result
}

pub fn vectorize<T: api::RecordMeta>(record: &T, add_x0: bool) -> Vec<api::Number> {
    let mut result = Vec::new();
    if add_x0 {
        result.push(1.0);
    }
    for f in 0..record.feature_count() {
        match record.feature_type(f) {
            api::FeatureType::Number => {
                if let Some(value) = record.number_value(f) {
                    result.push(value);
                }
            },
            api::FeatureType::Boolean => {
                if let Some(value) = record.bool_value(f) {
                    result.push(if value {0.0} else {1.0});
                }
            },
            api::FeatureType::Category => {
                if let Some(value) = record.category_value(f) {
                    for v in 0..record.category_count(f) {
                        result.push(if v as u32 == value {1.0} else {0.0});
                    };
                };  
            }
        };
    }
    result
}