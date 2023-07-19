use std::env;
use std::process::Command;

fn main() {
    #[cfg(target_arch = "x86_64")]
    {
        build_x86_64();
    }
    
    println!("cargo:rerun-if-changed=build.rs");
}

fn build_x86_64() {
    let build_dir = "./build";
    let build_dir_arg = env::current_dir().unwrap().join(build_dir);
    let asm_files: [&str; 1] = ["src/arch/x86_64/bootstrap.nasm"];
    let arch = "elf64";
    let arch_arg: String = "-f".to_owned() + arch;

    for file in asm_files {
        let object_file = file.split("/").last().unwrap().split(".").nth(0).unwrap().to_owned();
        let object_file_path = build_dir_arg.join(object_file + ".o");

        Command::new("nasm")
            .arg(&arch_arg[..])
            .arg(file)
            .args(&["-o", object_file_path.to_str().unwrap()])
            .status().unwrap();
    }
}