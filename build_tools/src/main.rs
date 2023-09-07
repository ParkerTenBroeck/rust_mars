use std::{path::{PathBuf, Path}, process::Command, str::FromStr};

use asm_pre_processor::create_asm;

pub mod asm_pre_processor;

fn main() {
    let args = std::env::args().skip(1).peekable(); // skip executable name

    let mut run = false;
    let mut build = false;
    let mut clean = false;
    // let mut release = false;
    let mut unreconized = Vec::new();

    for string in args {
        match string.as_str().trim() {
            "run" => {
                run = true;
                build = true;
            }
            "build" => {
                build = true;
            }
            "clean" => {
                clean = true;
            }
            _ => {
                unreconized.push(string);
            }
        }
    }

    if clean {
        let mut run_cmd = Command::new("cargo");
        run_cmd.arg("clean");
        assert!(run_cmd.status().unwrap().success());
    }

    let mut obj = None;
    // let mut bin = None;
    let mut asm = None;
    if build {
        let obj_t = build_vm_binary("main");
        // bin = Some(create_raw_binary("main"));
        asm = Some(create_assembly(&obj_t));

        obj = Some(obj_t);
    }

    if run {
        let mut run_cmd = Command::new("java");
        run_cmd.arg("-jar");
        let mut path = workspace_path();
        path.push("Mars4_5.jar");
        run_cmd.arg(path.to_str().expect("Failed to make MARS jar path"));
        
        run_cmd.arg(asm.as_ref().map(|f|f.to_str()).flatten().ok_or("Failed to get/make asm path").unwrap());

        assert!(run_cmd.status().is_ok())
    }
}

fn create_assembly(obj_t: &Path) -> PathBuf {
    create_asm(obj_t).expect("Faield to create assembly file")
}

pub fn workspace_path() -> PathBuf {
    let mut run_cmd = Command::new("cargo");
    run_cmd.arg("locate-project");
    run_cmd.arg("--message-format");
    run_cmd.arg("plain");
    run_cmd.arg("--workspace");

    let out = run_cmd.output().unwrap();
    assert!(out.status.success());
    let path = std::str::from_utf8(&out.stdout).unwrap();
    let path = path.trim();
    let path = path.split('\n').last().unwrap();
    let mut path = PathBuf::from_str(path).unwrap();
    path.pop();
    path
}

pub fn build_vm_binary(name: &str) -> PathBuf {
    let mut run_cmd = Command::new("cargo");
    run_cmd.current_dir(std::env::current_dir().unwrap());

    run_cmd
    .arg("+nightly")
        .arg("rustc")
        .arg("--release")
        .arg("--package")
        .arg(name)
        .arg("--target")
        .arg("mips.json")
        .arg("-Zbuild-std=core,compiler_builtins")
        .arg("-Zbuild-std-features=compiler-builtins-mem")
        .arg("--")
        .arg("-C")
        .arg("opt-level=z")
        ;

    assert!(run_cmd.status().unwrap().success());

    let mut path = workspace_path();
    path.push("target");
    path.push("mips");
    path.push("release");
    path.push(name);
    path
}

// pub fn create_raw_binary(name: &str) -> PathBuf {
//     let llvm_tools = llvm_tools::LlvmTools::new().unwrap();
//     let objcopy = llvm_tools.tool(&llvm_tools::exe("llvm-objcopy")).unwrap();

//     let mut run_cmd = Command::new(objcopy);
//     let mut path = workspace_path();
//     path.push("target");
//     path.push("mips");
//     path.push("release");
//     run_cmd.current_dir(path.clone());

//     run_cmd
//         .arg("-O")
//         .arg("binary")
//         .arg("-I")
//         .arg("elf32-big")
//         .arg(&format!("./{}", name))
//         .arg(&format!("./{}.bin", name));

//     assert!(run_cmd.status().unwrap().success());

//     path.push(&format!("{}.bin", name));
//     path
// }
