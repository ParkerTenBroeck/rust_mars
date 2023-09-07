use std::{ffi::OsStr, io::Write, path::PathBuf, process::Command, str::FromStr};

use walkdir::WalkDir;

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

    let mut bin = None;
    if build {
        build_vm_binary("main");
        bin = Some(create_raw_binary("main"));

    }

    if run {
        // let mut run_cmd = Command::new("java");
        // let mut path = workspace_path();
        // path.push("java_rt");
        // path.push("out");
        // run_cmd.current_dir(path);
        // run_cmd.arg("-jar");
        // run_cmd.arg("JavaRT.jar");
        todo!("something with this {:#?}", bin);

        // assert!(run_cmd.status().unwrap().success());
    }
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

pub fn build_vm_binary(name: &str) {
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
        // .arg("--")
        // .arg("--emit")
        // .arg("asm")
        ;

    assert!(run_cmd.status().unwrap().success());
}

pub fn create_raw_binary(name: &str) -> PathBuf {
    let llvm_tools = llvm_tools::LlvmTools::new().unwrap();
    let objcopy = llvm_tools.tool(&llvm_tools::exe("llvm-objcopy")).unwrap();

    let mut run_cmd = Command::new(objcopy);
    let mut path = workspace_path();
    path.push("target");
    path.push("mips");
    path.push("release");
    run_cmd.current_dir(path.clone());

    run_cmd
        .arg("-O")
        .arg("binary")
        .arg("-I")
        .arg("elf32-big")
        .arg(&format!("./{}", name))
        .arg(&format!("./{}.bin", name));

    assert!(run_cmd.status().unwrap().success());

    path.push(&format!("{}.bin", name));
    path
}
