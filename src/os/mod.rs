#!/usr/bin/env rust

use std::process::Command;
use crate::strings::split;

fn runcmd<'a>(command:String) -> Vec<String> {
    let args= split(command," ");
    if args.len() == 0 {
        return vec![]
    }
    let mut cmd = Command::new(&args[0]);
    let output = cmd.args(&args[1..]).output();
    
    return vec![String::from_utf8_lossy(&output.unwrap().stdout).into_owned()]
}

pub fn runcmds<'a>(commands:&'a str) -> Vec<String> {
    let cmds = split(commands,";");
    let mut outputs:Vec<String> = vec![];
    for cmd in cmds {
        let output = runcmd(cmd);
        if output.len() > 0 {
            outputs.push(output[0].clone());
        }
    }
    return outputs
}

use std::fs::File;
use std::io::Write;

pub fn list2file(filepath:&str,items:Vec<&str>){
    let mut file = File::create(filepath).unwrap();
    let fullstring= items.join("\n");
    if let Err(e) = writeln!(&mut file,"{}",fullstring){
        println!("Writing error: {}", e.to_string()); 
    }
}

use std::fs::read_to_string;

pub fn file2list<'a>(filepath:&'a str) -> Vec<String> {
    // let mut file = File::open(filepath).expect("unable to read file.");
    // let mut data = Vec::new();
    // file.read_to_string(&mut data).expect("unable to read string");
    let data:String = read_to_string(filepath).expect("unable to read file.");
    let output:Vec<String> = split(data,"\n");

    return output

}