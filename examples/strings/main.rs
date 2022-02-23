#!/usr/bin/env rust

use rustutils::strings::string2list;

fn main(){
    for item in string2list("a,b,c",",") {
        print!("{}\n",item);
    }
}