fn main() {
    let v = vec![1, 2, 3];
    print1(&v);
}

fn print1(v: &[i32]) {
    print2(v);
}

fn print2(v: &[i32]) {
    println!("{:?}", v);
}
