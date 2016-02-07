use super::super::api;

pub struct Record {
    pub sepal_length: api::Number,
    pub sepal_width: api::Number,
    pub petal_length: api::Number,
    pub petal_width: api::Number
}

impl api::RecordMeta for Record {

    fn feature_count(&self) -> usize {
        4
    }

    fn feature_name(&self, feature: usize) -> String {
        match feature {
            0 => "Sepal Length".to_string(),
            1 => "Sepal Width".to_string(),
            2 => "Petal Length".to_string(),
            3 => "Petal Width".to_string(),
            _ => panic!("Unknown feature")
        }
    }

    fn feature_type(&self, feature: usize) -> api::FeatureType {
        match feature {
            0 => api::FeatureType::Number,
            1 => api::FeatureType::Number,
            2 => api::FeatureType::Number,
            3 => api::FeatureType::Number,
            _ => panic!("Unknown feature")
        }   
    }

    fn category_count(&self, _: usize) -> usize {
        panic!("Doesn't have category features")
    }

    fn number_value(&self, feature: usize) -> api::Number {
        match feature {
            0 => self.sepal_length,
            1 => self.sepal_width,
            2 => self.petal_length,
            3 => self.petal_width,
            _ => panic!("Unknown feature")
        }   
    }

    fn category_value(&self, _: usize) -> api::Category {
        panic!("Doesn't have category features")
    }

    fn bool_value(&self, _: usize) -> bool {
        panic!("Doesn't have bool features")
    }

}

pub const SETOSA: api::Target = 0;
pub const VERSICOLOR: api::Target = 1;
pub const VIRGINICA: api::Target = 2;

pub fn read_data() -> api::DataSet<Record> {
    let result = api::DataSet{
        records: vec![
            (Record{sepal_length: 5.1, sepal_width: 3.5, petal_length: 1.4, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.9, sepal_width: 3.0, petal_length: 1.4, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.7, sepal_width: 3.2, petal_length: 1.3, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.6, sepal_width: 3.1, petal_length: 1.5, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.0, sepal_width: 3.6, petal_length: 1.4, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.4, sepal_width: 3.9, petal_length: 1.7, petal_width: 0.4}, SETOSA),
            (Record{sepal_length: 4.6, sepal_width: 3.4, petal_length: 1.4, petal_width: 0.3}, SETOSA),
            (Record{sepal_length: 5.0, sepal_width: 3.4, petal_length: 1.5, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.4, sepal_width: 2.9, petal_length: 1.4, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.9, sepal_width: 3.1, petal_length: 1.5, petal_width: 0.1}, SETOSA),
            (Record{sepal_length: 5.4, sepal_width: 3.7, petal_length: 1.5, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.8, sepal_width: 3.4, petal_length: 1.6, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.8, sepal_width: 3.0, petal_length: 1.4, petal_width: 0.1}, SETOSA),
            (Record{sepal_length: 4.3, sepal_width: 3.0, petal_length: 1.1, petal_width: 0.1}, SETOSA),
            (Record{sepal_length: 5.8, sepal_width: 4.0, petal_length: 1.2, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.7, sepal_width: 4.4, petal_length: 1.5, petal_width: 0.4}, SETOSA),
            (Record{sepal_length: 5.4, sepal_width: 3.9, petal_length: 1.3, petal_width: 0.4}, SETOSA),
            (Record{sepal_length: 5.1, sepal_width: 3.5, petal_length: 1.4, petal_width: 0.3}, SETOSA),
            (Record{sepal_length: 5.7, sepal_width: 3.8, petal_length: 1.7, petal_width: 0.3}, SETOSA),
            (Record{sepal_length: 5.1, sepal_width: 3.8, petal_length: 1.5, petal_width: 0.3}, SETOSA),
            (Record{sepal_length: 5.4, sepal_width: 3.4, petal_length: 1.7, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.1, sepal_width: 3.7, petal_length: 1.5, petal_width: 0.4}, SETOSA),
            (Record{sepal_length: 4.6, sepal_width: 3.6, petal_length: 1.0, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.1, sepal_width: 3.3, petal_length: 1.7, petal_width: 0.5}, SETOSA),
            (Record{sepal_length: 4.8, sepal_width: 3.4, petal_length: 1.9, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.0, sepal_width: 3.0, petal_length: 1.6, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.0, sepal_width: 3.4, petal_length: 1.6, petal_width: 0.4}, SETOSA),
            (Record{sepal_length: 5.2, sepal_width: 3.5, petal_length: 1.5, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.2, sepal_width: 3.4, petal_length: 1.4, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.7, sepal_width: 3.2, petal_length: 1.6, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.8, sepal_width: 3.1, petal_length: 1.6, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.4, sepal_width: 3.4, petal_length: 1.5, petal_width: 0.4}, SETOSA),
            (Record{sepal_length: 5.2, sepal_width: 4.1, petal_length: 1.5, petal_width: 0.1}, SETOSA),
            (Record{sepal_length: 5.5, sepal_width: 4.2, petal_length: 1.4, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.9, sepal_width: 3.1, petal_length: 1.5, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.0, sepal_width: 3.2, petal_length: 1.2, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.5, sepal_width: 3.5, petal_length: 1.3, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.9, sepal_width: 3.6, petal_length: 1.4, petal_width: 0.1}, SETOSA),
            (Record{sepal_length: 4.4, sepal_width: 3.0, petal_length: 1.3, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.1, sepal_width: 3.4, petal_length: 1.5, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.0, sepal_width: 3.5, petal_length: 1.3, petal_width: 0.3}, SETOSA),
            (Record{sepal_length: 4.5, sepal_width: 2.3, petal_length: 1.3, petal_width: 0.3}, SETOSA),
            (Record{sepal_length: 4.4, sepal_width: 3.2, petal_length: 1.3, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.0, sepal_width: 3.5, petal_length: 1.6, petal_width: 0.6}, SETOSA),
            (Record{sepal_length: 5.1, sepal_width: 3.8, petal_length: 1.9, petal_width: 0.4}, SETOSA),
            (Record{sepal_length: 4.8, sepal_width: 3.0, petal_length: 1.4, petal_width: 0.3}, SETOSA),
            (Record{sepal_length: 5.1, sepal_width: 3.8, petal_length: 1.6, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 4.6, sepal_width: 3.2, petal_length: 1.4, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.3, sepal_width: 3.7, petal_length: 1.5, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 5.0, sepal_width: 3.3, petal_length: 1.4, petal_width: 0.2}, SETOSA),
            (Record{sepal_length: 7.0, sepal_width: 3.2, petal_length: 4.7, petal_width: 1.4}, VERSICOLOR),
            (Record{sepal_length: 6.4, sepal_width: 3.2, petal_length: 4.5, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 6.9, sepal_width: 3.1, petal_length: 4.9, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 5.5, sepal_width: 2.3, petal_length: 4.0, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 6.5, sepal_width: 2.8, petal_length: 4.6, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 5.7, sepal_width: 2.8, petal_length: 4.5, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 6.3, sepal_width: 3.3, petal_length: 4.7, petal_width: 1.6}, VERSICOLOR),
            (Record{sepal_length: 4.9, sepal_width: 2.4, petal_length: 3.3, petal_width: 1.0}, VERSICOLOR),
            (Record{sepal_length: 6.6, sepal_width: 2.9, petal_length: 4.6, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 5.2, sepal_width: 2.7, petal_length: 3.9, petal_width: 1.4}, VERSICOLOR),
            (Record{sepal_length: 5.0, sepal_width: 2.0, petal_length: 3.5, petal_width: 1.0}, VERSICOLOR),
            (Record{sepal_length: 5.9, sepal_width: 3.0, petal_length: 4.2, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 6.0, sepal_width: 2.2, petal_length: 4.0, petal_width: 1.0}, VERSICOLOR),
            (Record{sepal_length: 6.1, sepal_width: 2.9, petal_length: 4.7, petal_width: 1.4}, VERSICOLOR),
            (Record{sepal_length: 5.6, sepal_width: 2.9, petal_length: 3.6, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 6.7, sepal_width: 3.1, petal_length: 4.4, petal_width: 1.4}, VERSICOLOR),
            (Record{sepal_length: 5.6, sepal_width: 3.0, petal_length: 4.5, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 5.8, sepal_width: 2.7, petal_length: 4.1, petal_width: 1.0}, VERSICOLOR),
            (Record{sepal_length: 6.2, sepal_width: 2.2, petal_length: 4.5, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 5.6, sepal_width: 2.5, petal_length: 3.9, petal_width: 1.1}, VERSICOLOR),
            (Record{sepal_length: 5.9, sepal_width: 3.2, petal_length: 4.8, petal_width: 1.8}, VERSICOLOR),
            (Record{sepal_length: 6.1, sepal_width: 2.8, petal_length: 4.0, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 6.3, sepal_width: 2.5, petal_length: 4.9, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 6.1, sepal_width: 2.8, petal_length: 4.7, petal_width: 1.2}, VERSICOLOR),
            (Record{sepal_length: 6.4, sepal_width: 2.9, petal_length: 4.3, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 6.6, sepal_width: 3.0, petal_length: 4.4, petal_width: 1.4}, VERSICOLOR),
            (Record{sepal_length: 6.8, sepal_width: 2.8, petal_length: 4.8, petal_width: 1.4}, VERSICOLOR),
            (Record{sepal_length: 6.7, sepal_width: 3.0, petal_length: 5.0, petal_width: 1.7}, VERSICOLOR),
            (Record{sepal_length: 6.0, sepal_width: 2.9, petal_length: 4.5, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 5.7, sepal_width: 2.6, petal_length: 3.5, petal_width: 1.0}, VERSICOLOR),
            (Record{sepal_length: 5.5, sepal_width: 2.4, petal_length: 3.8, petal_width: 1.1}, VERSICOLOR),
            (Record{sepal_length: 5.5, sepal_width: 2.4, petal_length: 3.7, petal_width: 1.0}, VERSICOLOR),
            (Record{sepal_length: 5.8, sepal_width: 2.7, petal_length: 3.9, petal_width: 1.2}, VERSICOLOR),
            (Record{sepal_length: 6.0, sepal_width: 2.7, petal_length: 5.1, petal_width: 1.6}, VERSICOLOR),
            (Record{sepal_length: 5.4, sepal_width: 3.0, petal_length: 4.5, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 6.0, sepal_width: 3.4, petal_length: 4.5, petal_width: 1.6}, VERSICOLOR),
            (Record{sepal_length: 6.7, sepal_width: 3.1, petal_length: 4.7, petal_width: 1.5}, VERSICOLOR),
            (Record{sepal_length: 6.3, sepal_width: 2.3, petal_length: 4.4, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 5.6, sepal_width: 3.0, petal_length: 4.1, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 5.5, sepal_width: 2.5, petal_length: 4.0, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 5.5, sepal_width: 2.6, petal_length: 4.4, petal_width: 1.2}, VERSICOLOR),
            (Record{sepal_length: 6.1, sepal_width: 3.0, petal_length: 4.6, petal_width: 1.4}, VERSICOLOR),
            (Record{sepal_length: 5.8, sepal_width: 2.6, petal_length: 4.0, petal_width: 1.2}, VERSICOLOR),
            (Record{sepal_length: 5.0, sepal_width: 2.3, petal_length: 3.3, petal_width: 1.0}, VERSICOLOR),
            (Record{sepal_length: 5.6, sepal_width: 2.7, petal_length: 4.2, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 5.7, sepal_width: 3.0, petal_length: 4.2, petal_width: 1.2}, VERSICOLOR),
            (Record{sepal_length: 5.7, sepal_width: 2.9, petal_length: 4.2, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 6.2, sepal_width: 2.9, petal_length: 4.3, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 5.1, sepal_width: 2.5, petal_length: 3.0, petal_width: 1.1}, VERSICOLOR),
            (Record{sepal_length: 5.7, sepal_width: 2.8, petal_length: 4.1, petal_width: 1.3}, VERSICOLOR),
            (Record{sepal_length: 6.3, sepal_width: 3.3, petal_length: 6.0, petal_width: 2.5}, VIRGINICA),
            (Record{sepal_length: 5.8, sepal_width: 2.7, petal_length: 5.1, petal_width: 1.9}, VIRGINICA),
            (Record{sepal_length: 7.1, sepal_width: 3.0, petal_length: 5.9, petal_width: 2.1}, VIRGINICA),
            (Record{sepal_length: 6.3, sepal_width: 2.9, petal_length: 5.6, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 6.5, sepal_width: 3.0, petal_length: 5.8, petal_width: 2.2}, VIRGINICA),
            (Record{sepal_length: 7.6, sepal_width: 3.0, petal_length: 6.6, petal_width: 2.1}, VIRGINICA),
            (Record{sepal_length: 4.9, sepal_width: 2.5, petal_length: 4.5, petal_width: 1.7}, VIRGINICA),
            (Record{sepal_length: 7.3, sepal_width: 2.9, petal_length: 6.3, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 6.7, sepal_width: 2.5, petal_length: 5.8, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 7.2, sepal_width: 3.6, petal_length: 6.1, petal_width: 2.5}, VIRGINICA),
            (Record{sepal_length: 6.5, sepal_width: 3.2, petal_length: 5.1, petal_width: 2.0}, VIRGINICA),
            (Record{sepal_length: 6.4, sepal_width: 2.7, petal_length: 5.3, petal_width: 1.9}, VIRGINICA),
            (Record{sepal_length: 6.8, sepal_width: 3.0, petal_length: 5.5, petal_width: 2.1}, VIRGINICA),
            (Record{sepal_length: 5.7, sepal_width: 2.5, petal_length: 5.0, petal_width: 2.0}, VIRGINICA),
            (Record{sepal_length: 5.8, sepal_width: 2.8, petal_length: 5.1, petal_width: 2.4}, VIRGINICA),
            (Record{sepal_length: 6.4, sepal_width: 3.2, petal_length: 5.3, petal_width: 2.3}, VIRGINICA),
            (Record{sepal_length: 6.5, sepal_width: 3.0, petal_length: 5.5, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 7.7, sepal_width: 3.8, petal_length: 6.7, petal_width: 2.2}, VIRGINICA),
            (Record{sepal_length: 7.7, sepal_width: 2.6, petal_length: 6.9, petal_width: 2.3}, VIRGINICA),
            (Record{sepal_length: 6.0, sepal_width: 2.2, petal_length: 5.0, petal_width: 1.5}, VIRGINICA),
            (Record{sepal_length: 6.9, sepal_width: 3.2, petal_length: 5.7, petal_width: 2.3}, VIRGINICA),
            (Record{sepal_length: 5.6, sepal_width: 2.8, petal_length: 4.9, petal_width: 2.0}, VIRGINICA),
            (Record{sepal_length: 7.7, sepal_width: 2.8, petal_length: 6.7, petal_width: 2.0}, VIRGINICA),
            (Record{sepal_length: 6.3, sepal_width: 2.7, petal_length: 4.9, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 6.7, sepal_width: 3.3, petal_length: 5.7, petal_width: 2.1}, VIRGINICA),
            (Record{sepal_length: 7.2, sepal_width: 3.2, petal_length: 6.0, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 6.2, sepal_width: 2.8, petal_length: 4.8, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 6.1, sepal_width: 3.0, petal_length: 4.9, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 6.4, sepal_width: 2.8, petal_length: 5.6, petal_width: 2.1}, VIRGINICA),
            (Record{sepal_length: 7.2, sepal_width: 3.0, petal_length: 5.8, petal_width: 1.6}, VIRGINICA),
            (Record{sepal_length: 7.4, sepal_width: 2.8, petal_length: 6.1, petal_width: 1.9}, VIRGINICA),
            (Record{sepal_length: 7.9, sepal_width: 3.8, petal_length: 6.4, petal_width: 2.0}, VIRGINICA),
            (Record{sepal_length: 6.4, sepal_width: 2.8, petal_length: 5.6, petal_width: 2.2}, VIRGINICA),
            (Record{sepal_length: 6.3, sepal_width: 2.8, petal_length: 5.1, petal_width: 1.5}, VIRGINICA),
            (Record{sepal_length: 6.1, sepal_width: 2.6, petal_length: 5.6, petal_width: 1.4}, VIRGINICA),
            (Record{sepal_length: 7.7, sepal_width: 3.0, petal_length: 6.1, petal_width: 2.3}, VIRGINICA),
            (Record{sepal_length: 6.3, sepal_width: 3.4, petal_length: 5.6, petal_width: 2.4}, VIRGINICA),
            (Record{sepal_length: 6.4, sepal_width: 3.1, petal_length: 5.5, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 6.0, sepal_width: 3.0, petal_length: 4.8, petal_width: 1.8}, VIRGINICA),
            (Record{sepal_length: 6.9, sepal_width: 3.1, petal_length: 5.4, petal_width: 2.1}, VIRGINICA),
            (Record{sepal_length: 6.7, sepal_width: 3.1, petal_length: 5.6, petal_width: 2.4}, VIRGINICA),
            (Record{sepal_length: 6.9, sepal_width: 3.1, petal_length: 5.1, petal_width: 2.3}, VIRGINICA),
            (Record{sepal_length: 5.8, sepal_width: 2.7, petal_length: 5.1, petal_width: 1.9}, VIRGINICA),
            (Record{sepal_length: 6.8, sepal_width: 3.2, petal_length: 5.9, petal_width: 2.3}, VIRGINICA),
            (Record{sepal_length: 6.7, sepal_width: 3.3, petal_length: 5.7, petal_width: 2.5}, VIRGINICA),
            (Record{sepal_length: 6.7, sepal_width: 3.0, petal_length: 5.2, petal_width: 2.3}, VIRGINICA),
            (Record{sepal_length: 6.3, sepal_width: 2.5, petal_length: 5.0, petal_width: 1.9}, VIRGINICA),
            (Record{sepal_length: 6.5, sepal_width: 3.0, petal_length: 5.2, petal_width: 2.0}, VIRGINICA),
            (Record{sepal_length: 6.2, sepal_width: 3.4, petal_length: 5.4, petal_width: 2.3}, VIRGINICA),
            (Record{sepal_length: 5.9, sepal_width: 3.0, petal_length: 5.1, petal_width: 1.8}, VIRGINICA)
        ],
        target_count: 3
    };
    result
}
