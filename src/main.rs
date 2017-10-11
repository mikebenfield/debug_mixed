#![feature(abi_sysv64)]

extern "sysv64" {
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
