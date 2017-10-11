
extern "C" {
    fn c_function();
    fn asm_function();
}


fn main() {
    println!("Hello, world!");
    unsafe {
        c_function();
        asm_function();
    }
}
