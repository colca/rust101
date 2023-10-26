fn vec_min(v: &Vec<i32>) -> Option<i32> {
    use std::cmp;

    let mut min = None;

    for e in v.iter() {
        min = Some (match min {
            None => *e,
            Some(n) => cmp::min(n, *e)
        });
    }
    println!("The min element is {:?}", min);
    min
}

fn vec_inc(v: &mut Vec<i32>) {
    for e in v.iter_mut() {
        *e += 1
    }
}

pub fn main() {
    let mut v = vec![1, 2, 3, 4];
    let first = &v[0];
    vec_min(&v);
    println!("The first element is {}", *first);
    vec_inc(&mut v);
    // println!("The first element is {}", *first);
}