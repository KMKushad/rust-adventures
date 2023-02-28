fn main() {
    let mut v = vec![1, 2, 5, 2, 7, 9, 10];
    v.sort();
    println!("{:?}", v);

    if v.len() % 2 == 0 {
        let mid_index: f64 = (v.len() - 1) as f64 / 2.0;
        let median: f64 = (v[(mid_index + 0.5) as usize] + v[(mid_index - 0.5) as usize]) as f64 / 2.0;
        println!("{median}");
    } else {
        let mid_index = (v.len() - 1) / 2;
        println!("{}", v[mid_index]);
    }
}
