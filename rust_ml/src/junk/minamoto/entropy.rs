use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::hash::Hash;
use std::rc::Rc;


fn counters<T, C:Hash+Eq> (dataset:&Vec<(Rc<T>,Rc<C>)>) -> (HashMap<Rc<C>, u64>, f64) {
    let mut counters = HashMap::new();
    let count = dataset.len() as f64;

    for d in dataset {
        let num;
        match counters.get(d.1.as_ref()) {
            Some(n) => num = n + 1,
            None => num = 1
        };
        counters.insert(d.1.clone(), num);
    };
    (counters,count)
}

fn entropy4iter<C:Eq+Hash>(iter:Iter<Rc<C>,u64>, count:f64) -> f64 {
    iter.fold(0f64, |x, y| {
        let t:f64 = (*y.1 as f64)/count;
        x + t * t.log2() 
    })
        
}

pub fn entropy<T, C:Eq+Hash>(dataset:&Vec<(Rc<T>,Rc<C>)>) -> f64 {
    let (counters,count) = counters(dataset);
    - entropy4iter(counters.iter(), count)
}

pub fn gain<T, C:Eq+Hash>(dataset:&Vec<(Rc<T>,Rc<C>)>, class:Rc<C>) -> f64 {
    let (mut counters,count) = counters(dataset);
    counters.remove(class.as_ref());
    - entropy4iter(counters.iter(), count)
}
