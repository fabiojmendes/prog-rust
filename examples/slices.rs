use std::fmt::Display;

fn str_arg(s: &str) {
    println!("str arg: {}", s);
}

fn string_arg(s: &String) {
    println!("String arg: {}", s);
}

fn generic_arg<'a, T>(s: T)
where
    T: Into<&'a str> + Display,
{
    println!("Generic arg: {}", s);
}

fn main() {
    let s = String::from("test");
    str_arg(&s);
    string_arg(&s);
    generic_arg(&*s);
    generic_arg(&s[..]);
}
