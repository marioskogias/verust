#[link(name = "mylib")]
extern "C" {
    fn marios_print();
}


fn main() {
    println!("Hello, world!");
    unsafe {marios_print();}
}
