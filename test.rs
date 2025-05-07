fn main() {
    let mut s = String::from("Hello World");
    let s2 = s.clone();
    s = s.replace("World", "Rust");
    println!("{}", s);
    println!("{}", s2);
}
