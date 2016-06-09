use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let arch_src_file =
        format!("src/arch/{}/jump.s",
                env::var("TARGET").unwrap().split("-").next().unwrap());
    if !(Command::new("as").args(&["-o", &(out_dir.clone() + "/jump.o"),
                                   &arch_src_file])
                           .status().unwrap().success() &&
         Command::new("ar").args(&["-crus",
                                   &(out_dir.clone() + "/libjump.a"),
                                   &(out_dir.clone() + "/jump.o")])
                            .status().unwrap().success()) {
      panic!("failed");
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
}
