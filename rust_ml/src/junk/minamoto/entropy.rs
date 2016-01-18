
pub trait Classified<T> {
    fn class(&self) -> T;
}

pub fn entropy<C:PartialEq, T:Classified<C>>(data:&Vec<T>, classes:&Vec<C>) -> f64 {
    let mut sum: f64 = 0f64;
    for c in classes {
        let mut p: f64 = 0f64;
        for d in data {
            if d.class() == *c {
                p = p + 1f64;
            }
        }
        p = p / (data.len() as f64);
        p = - p * p.log2();
        sum += p;
    }
    sum
}
