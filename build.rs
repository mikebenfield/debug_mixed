use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("clang").args(&["src/file.c", "-c", "-g", "-o"])
                       .arg(&format!("{}/file.o", out_dir))
                       .status().unwrap();
    Command::new("clang").args(&["src/file2.s", "-c", "-g", "-o"])
                       .arg(&format!("{}/file2.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libfile.a", "file.o", "file2.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=file");
}
