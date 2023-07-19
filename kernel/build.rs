use std::env;
use std::process::Command;

fn main() {
    #[cfg(target_arch = "x86_64")]
    {
        build_x86_64();
    }

    #[cfg(target_arch = "aarch64")]
    {
        build_aarch68();
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn build_x86_64() {
    let build_dir = "../build";
    let build_dir_arg = env::current_dir().unwrap().join(build_dir);
    let asm_files: [&str; 1] = ["src/arch/x86_64/boot/s2.nasm"];
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

fn build_aarch68() {
    let asm_files: [&str; 1] = ["src/arch/aarch64/s2.S"];
    let build_dir = "../build";
    let arch = "aarch64-elf";

    let build_dir_arg = env::current_dir().unwrap().join(build_dir);
    let arch_arg: String = "--target".to_owned() + arch;

    for file in asm_files {
        let object_file = file.split("/").last().unwrap().split(".").nth(0).unwrap().to_owned();
        let object_file_path = build_dir_arg.join(object_file + ".o");

        Command::new("clang")
            .arg(&arch_arg[..])
            .arg(file)
            .args(&["-o", object_file_path.to_str().unwrap()])
            .status().unwrap();
    }
}