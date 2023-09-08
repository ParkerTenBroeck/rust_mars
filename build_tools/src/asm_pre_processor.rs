use std::{collections::HashMap, path::{Path, PathBuf}, process::Command};

use regex::Regex;


pub fn pre_processes_data_seg(raw: &[u8]) -> Result<String, Box<dyn std::error::Error>>{
    if raw.len() == 0{
        return Ok(String::new())
    }
    // let mut val = String::new();
    let mut val = ".byte ".to_owned();

    // std::str::f

    for byte in raw{
        // val.push_str(".byte ");
        val.push_str(&format!("{:}", *byte as i8));
        val.push(' ');
    }

    Ok(val)
}

pub fn create_asm(binary: &Path) -> Result<PathBuf, Box<dyn std::error::Error>>{
    let mut owned = binary.to_owned();
    owned.pop();
    owned.push("out.asm");

    let mut final_out = {
        let mut run_cmd = Command::new("mipsel-linux-gnu-objdump");
        run_cmd.arg("-d");
        run_cmd.arg(binary.to_str().expect("Invalid Path"));
        run_cmd.arg("-M");
        run_cmd.arg("reg-names=numeric");
        run_cmd.arg("--show-all-symbols");
        run_cmd.arg("--disassemble-zeroes");
        run_cmd.arg("-M");
        run_cmd.arg("no-aliases");
        run_cmd.arg("--no-show-raw-insn");
    
    
        let output = run_cmd.output()?;
        let output = String::from_utf8(output.stdout)?;
        // panic!("{:#?}", output);
        pre_processes_text(output)?
    };
    {
        let mut run_cmd = Command::new("mipsel-linux-gnu-objcopy");
        // run_cmd.arg("-d");
        run_cmd.arg(binary.to_str().expect("Invalid Path"));
        run_cmd.arg("--dump-section");
        run_cmd.arg(".got=/dev/stdout");
        // run_cmd.arg("--show-all-symbols");
        // run_cmd.arg("--demangle=rust");
        // run_cmd.arg("--no-show-raw-insn");
    
    
        let output = run_cmd.output()?;
        let data = pre_processes_data_seg(&output.stdout)?;
        final_out.push_str("#.got \n.data\n");
        final_out.push_str(&data);
        final_out.push('\n');
    }
    {
        let mut run_cmd = Command::new("mipsel-linux-gnu-objcopy");
        // run_cmd.arg("-d");
        run_cmd.arg(binary.to_str().expect("Invalid Path"));
        run_cmd.arg("--dump-section");
        run_cmd.arg(".rodata=/dev/stdout");
        // run_cmd.arg("--show-all-symbols");
        // run_cmd.arg("--demangle=rust");
        // run_cmd.arg("--no-show-raw-insn");
    
    
        let output = run_cmd.output()?;
        let data = pre_processes_data_seg(&output.stdout)?;
        final_out.push_str("#.rodata \n.data\n");
        final_out.push_str(&data);
        final_out.push('\n');
    }
    {
        let mut run_cmd = Command::new("mipsel-linux-gnu-objcopy");
        // run_cmd.arg("-d");
        run_cmd.arg(binary.to_str().expect("Invalid Path"));
        run_cmd.arg("--dump-section");
        run_cmd.arg(".data=/dev/stdout");
        // run_cmd.arg("--show-all-symbols");
        // run_cmd.arg("--demangle=rust");
        // run_cmd.arg("--no-show-raw-insn");
    
    
        let output = run_cmd.output()?;
        let data = pre_processes_data_seg(&output.stdout)?;
        final_out.push_str("#.data \n.data\n");
        final_out.push_str(&data);
        final_out.push('\n');
    }
    {
        let mut run_cmd = Command::new("mipsel-linux-gnu-objcopy");
        // run_cmd.arg("-d");
        run_cmd.arg(binary.to_str().expect("Invalid Path"));
        run_cmd.arg("--dump-section");
        run_cmd.arg(".bss=/dev/stdout");
        // run_cmd.arg("--show-all-symbols");
        // run_cmd.arg("--demangle=rust");
        // run_cmd.arg("--no-show-raw-insn");
    
    
        let output = run_cmd.output()?;
        let data = pre_processes_data_seg(&output.stdout)?;
        final_out.push_str("#.bss \n.data\n");
        final_out.push_str(&data);
        final_out.push('\n');
    }


    // panic!("{:#?}", owned);
    std::fs::write(&owned, final_out)?;
            
    Ok(owned)
}

pub fn pre_processes_text(vals: String) -> Result<String, Box<dyn std::error::Error>>{
    let section_name;

    let mut iter = vals.split('\n').skip(4);
    
    let first = iter.next().ok_or("Expected at least one line")?;
    if let Some(val) = first.strip_prefix("Disassembly of section "){
        if let Some((section, _ )) = val.split_once(':'){
            section_name = section;
        }else{
            return Err("".into())
        }
    }else{
        return Err("".into())
    }

    let mut labels: HashMap<u32, Vec<String>> = HashMap::new();
    let mut instructions = Vec::new();
    
    let re = Regex::new(r"((.+)(\s+|,))([0-9a-fA-F]{1,8})\s*<(.*)>")?;
    let mut last_last_label = "first".into();
    let mut label_tmp = 0;

    for line in iter{
        if line.trim().is_empty(){
        }else{
            let (addr, rest) = line.split_at(8);
            let addr = u32::from_str_radix(addr.trim(), 16)?;
            let rest = rest.trim();
            if let Some(postfix) = rest.strip_prefix(":"){
                let line = if let Some(some) = re.captures(postfix){
                    let caps = some.iter().flatten().collect::<Vec::<_>>();
                    let thing = caps[1].as_str().trim();
                    let addr_lab = u32::from_str_radix(caps[4].as_str().trim(), 16)?;
                    let lab = caps[5].as_str().trim();
                    // println!("{:#?}", (thing, addr_lab, lab));
                    if lab.contains('+'){
                        let lab = format!("{last_last_label}_{label_tmp}");
                        let tmp  = format!("{} {}", thing, lab);
                        labels.entry(addr_lab).or_default().push(lab);
                        
                        label_tmp += 1;

                        (addr, tmp)
                    }else{
                        (addr, format!("{} {}", thing, lab))
                    }
                }else{
                    // let postfix = postfix.trim();
                    // if let Some((ins, oper)) = postfix.split_once(' '){
                        
                    // }else{

                    // }
                    // // match postfix
                    // if postfix == "sync"{

                    // }else{
                    //     instructions.push((addr, postfix.to_string()));
                    // }
                    (addr, postfix.trim().to_string())
                };
                let (ins, oper) = line.1.trim().split_once(|c: char|c.is_whitespace()).unwrap_or((&line.1, ""));
                
                match ins.trim(){
                    "teq" => {
                        
                        // let oper = oper.trim_end_matches(|c| matches!(c, ' '|'0'..='9',','));
                        let oper = &oper[..5];
                        instructions.push((line.0, format!("{ins} {oper}")));
                    }
                    "sync" => {}//skip
                    _ => {
                        instructions.push(line);
                    }
                }
            }else{
                label_tmp = 0;
                let rest = rest.trim();
                let rest = &rest[1..(rest.len() - 2)];
                // println!("{rest}");
                last_last_label = rest.to_owned();//rest.replace(":", "_").replace('<', "___").replace('>', "___").replace("&", "AND").replace(" ", "_");
                labels.entry(addr).or_default().push(last_last_label.to_owned());
            }
        }
    }

    let mut res = String::with_capacity(vals.len());

    res.push_str(section_name);
    res.push('\n');
    res.push('\n');

    for (addr, instruction) in instructions.into_iter(){
        if let Some(labels) = labels.remove(&addr){
            res.push('\n');
            for label in labels{
                res.push_str(&label);
                res.push(':');
                res.push('\n');
            }
        }
        res.push('\t');
        res.push_str(&instruction);
        res.push('\n');
    }

    Ok(res)
}