use std::env;
use std::process::Command;

fn main() {
    //let out_dir = env::var_os("OUT_DIR").expect("Something is really wrong. I could not find the OUT_DIR variable.");
    
    // Edit me!
    let asm_files: [&str; 1] = ["src/arch/x86_64/boot/s2.nasm"];
    let build_dir = "build";
    let tarch = "elf64";

    // Do not edit me.
    let build_dir_arg = env::current_dir().unwrap().join(build_dir);
    let arch_arg: String = "-f".to_owned() + tarch;
    
    for file in asm_files {
       	    let object_file = file.split("/").last().unwrap().split(".").nth(0).unwrap().to_owned();
	    let object_file_path = build_dir_arg.join(object_file + ".o");
	
	    Command::new("nasm")
	        .arg(&arch_arg[..])
	        .arg(file)
	        .args(&["-o", object_file_path.to_str().unwrap()])
	        .status().unwrap();

    }

    println!("cargo:rerun-if-changed=Cargo.lock");
}
