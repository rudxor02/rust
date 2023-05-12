fn main() {
    println!("{:#?}", a(Some(3)))
}

fn a(a: Option<i32>) -> Option<i32> {
    match a {
        None => None,
        Some(i) => Some(i + 3),
    }
}
