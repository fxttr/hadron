/*
 * Copyright (c) 2022, Florian Büstgens
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     1. Redistributions of source code must retain the above copyright
 *        notice, this list of conditions and the following disclaimer.
 *
 *     2. Redistributions in binary form must reproduce the above copyright notice,
 *        this list of conditions and the following disclaimer in the
 *        documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY Florian Büstgens ''AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL Florian Büstgens BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("Something is really wrong. I could not find the OUT_DIR variable.");
    
    // Edit me!
    let asm_files: [&str; 1] = ["src/arch/amd64/boot/s2.nasm"];
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

    println!("cargo:rerun-if-changed=build.rs");
}
