#!/usr/bin/env rust

use std::process::Command;
use crate::strings::string2list;

fn runcmd<'a>(command:&'a str) -> Vec<String> {
    let args:Vec<&str>= string2list(command," ");
    let mut cmd = Command::new(args[0]);
    let output = cmd.args(&args[1..]).output();
    
    return vec![String::from_utf8_lossy(&output.unwrap().stdout).into_owned()]
}

pub fn runcmds<'a>(commands:&'a str) -> Vec<String> {
    let cmds:Vec<&'a str> = string2list(commands,";");
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
    writeln!(&mut file,"{}",fullstring);
}