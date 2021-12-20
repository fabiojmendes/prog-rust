use std::fmt;

#[derive(Debug)]
enum Opt<T> {
    Null,
    Val(T),
}

#[derive(Debug)]
struct Container {
    id: u16,
    data: [u8; 16],
}

impl Container {
    fn new() -> Container {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        Container { id: 123, data }
    }
}

fn main() {
    let val = Opt::Val(&10);
    print_opt(val);

    let val2: Opt<&u64> = Opt::Null;
    print_opt(val2);

    let val3 = Opt::Val(&42);
    print_opt(val3);

    let mut rnd = Container::new();
    rnd.id += 32;
    rnd.data[0] = 10;
    let rnd_opt = Opt::Val(&rnd);
    print_opt(rnd_opt);

    // let vals = vec![Opt::Val(10), Opt::Val(32), Opt::Null];
    // print_vals(&vals);
}

#[inline(never)]
fn print_opt<T: fmt::Debug>(o: Opt<&T>) {
    println!("Hello val: {:?}", o);
}

// #[inline(never)]
// fn print_vals(vals: &[Opt<u32>]) {
//     for v in vals {
//         println!("Hello val: {:?}", v);
//     }
// }
