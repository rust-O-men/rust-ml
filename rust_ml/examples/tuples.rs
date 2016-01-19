fn main() {
    let test = vec![(1, 1), (2, 1), (2,2)];
    let result:Vec<i32> = test.iter().map(|x| x.1).collect();
    for r in result {
        println!("{}", r);
    }
}
