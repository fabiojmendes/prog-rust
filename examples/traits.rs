trait Printer {
    fn print(&self);
}

struct A {
    val: i32,
}

impl Printer for A {
    fn print(&self) {
        println!("A: {}", self.val);
    }
}

struct B {
    bar: f32,
}

impl Printer for B {
    fn print(&self) {
        println!("B: {}", self.bar);
    }
}

fn caller_impl(p: &impl Printer) {
    println!("Print from impl");
    p.print();
}

fn caller_bound<P: Printer>(p: &P) {
    println!("Print from bound");
    p.print();
}

fn caller_where<P>(p: &P)
where
    P: Printer,
{
    println!("Print from where");
    p.print();
}

fn caller_trait_obj(p: &dyn Printer) {
    println!("Print from trait obj");
    p.print();
}

///
/// Different ways of calling traits
///
fn main() {
    let a = A { val: 21 };
    let b = B { bar: 33.4 };

    caller_impl(&a);
    caller_impl(&b);

    caller_bound(&a);
    caller_bound(&b);

    caller_where(&a);
    caller_where(&b);

    caller_trait_obj(&a);
    caller_trait_obj(&b);

    println!("Printing list");
    let ab: Vec<&dyn Printer> = vec![&a, &b];
    for e in ab {
        caller_trait_obj(e);
    }
}
