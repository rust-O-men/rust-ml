
pub trait Classified<T> {
    fn class(&self) -> T;
}

pub fn entropy<C:PartialEq, T:Classified<C>>(data:&Vec<T>, classes:&Vec<C>) -> f64 {
    let mut sum: f64 = 0f64;
    for c in classes {
        let mut p: f64;
        p = data.iter().filter(|&x| x.class() == *c).fold(0f64, |x, _y| x + 1f64);
        p = p / (data.len() as f64);
        p = - p * p.log2();
        sum += p;
    }
    sum
}

pub fn gain<C:PartialEq+Clone, T:Classified<C>>(data:&Vec<T>, classes:&Vec<C>, class:&C) -> f64 {
    let target = vec![class.clone()];
    entropy(data, classes) - entropy(data, &target)
}
